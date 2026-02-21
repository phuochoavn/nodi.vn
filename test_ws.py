#!/usr/bin/env python3
"""Comprehensive WebSocket Support Chat E2E Test v3"""
import websocket, json, ssl, time, urllib.request, sys

SSL_OPTS = {'cert_reqs': ssl.CERT_NONE}
SSL_CTX = ssl.create_default_context()
SSL_CTX.check_hostname = False
SSL_CTX.verify_mode = ssl.CERT_NONE

LK = "NODI-WHS2-IU1V-7JVC"
ADMIN_TOKEN = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjM4LCJzdG9yZV9pZCI6MSwicm9sZSI6ImFkbWluIiwiZXhwIjoxNzcxNjM5MDE5LCJpYXQiOjE3NzE1NTI2MTl9.gRcJSqRLCkQ588s2xAbT9l97Up-98c94lql2sQ7C4_Q"
TICKET_ID = 8
BASE = "wss://nodi.vn"

results = []
msg_list = []

def test(name, passed, detail=""):
    status = "✅" if passed else "❌"
    results.append((name, passed))
    print(f"  {status} {name}" + (f" — {detail}" if detail else ""))

def recv_event(ws, expected_type, timeout=3):
    ws.settimeout(timeout)
    deadline = time.time() + timeout
    while time.time() < deadline:
        try:
            data = json.loads(ws.recv())
            if data.get("type") == expected_type:
                return data
        except: return None
    return None

UA = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 Chrome/120.0 Safari/537.36"

def api_get(path, token):
    req = urllib.request.Request(f"https://api.nodi.vn{path}",
        headers={"Authorization": f"Bearer {token}", "User-Agent": UA})
    return json.loads(urllib.request.urlopen(req, context=SSL_CTX).read())

def api_put(path, body, token):
    data = json.dumps(body).encode()
    req = urllib.request.Request(f"https://api.nodi.vn{path}", data=data, method="PUT",
        headers={"Content-Type": "application/json", "Authorization": f"Bearer {token}", "User-Agent": UA})
    return json.loads(urllib.request.urlopen(req, context=SSL_CTX).read())

# Ensure ticket is open
try: api_put(f"/api/admin/support/tickets/{TICKET_ID}/status", {"status": "open"}, ADMIN_TOKEN)
except: pass
time.sleep(1)

# ==== TEST 1: Connect both ====
print("=" * 60)
print("TEST 1: WS URL Format — query param auth")
print("=" * 60)
ws_c = websocket.create_connection(f"{BASE}/api/support/ws?ticket_id={TICKET_ID}&license_key={LK}", sslopt=SSL_OPTS, timeout=5)
test("Customer WS: license_key as query param", True, f"status={ws_c.status}")

ws_a = websocket.create_connection(f"{BASE}/api/support/ws?ticket_id={TICKET_ID}&token={ADMIN_TOKEN}", sslopt=SSL_OPTS, timeout=5)
test("Admin WS: JWT token as query param", True)

# Drain connect events
time.sleep(0.5)
ws_c.settimeout(0.3)
ws_a.settimeout(0.3)
try:
    while True: ws_c.recv()
except: pass
try:
    while True: ws_a.recv()
except: pass

# ==== TEST 2: Customer→Admin ====
print("\n" + "=" * 60)
print("TEST 2: Customer → Admin real-time message")
print("=" * 60)
ws_c.send(json.dumps({"type": "message", "text": "Hello from customer WS!"}))
time.sleep(0.5)
a_msg = recv_event(ws_a, "message")
test("Admin receives customer message", a_msg and a_msg["sender_type"] == "customer",
     f"id={a_msg.get('id')}, text='{a_msg.get('text')}'")
test("VN timezone (+07:00)", a_msg and "+07:00" in a_msg.get("created_at",""), a_msg.get("created_at") if a_msg else "")
# Drain customer echo
recv_event(ws_c, "message", timeout=1)

# ==== TEST 3: Admin→Customer ====
print("\n" + "=" * 60)
print("TEST 3: Admin → Customer real-time message")
print("=" * 60)
ws_a.send(json.dumps({"type": "message", "text": "Admin reply via WS!"}))
time.sleep(0.5)
c_msg = recv_event(ws_c, "message")
test("Customer receives admin message", c_msg and c_msg["sender_type"] == "admin",
     f"text='{c_msg.get('text') if c_msg else ''}'")
recv_event(ws_a, "message", timeout=1)

# ==== TEST 4: DB Persistence ====
print("\n" + "=" * 60)
print("TEST 4: DB Persistence")
print("=" * 60)
try:
    data = api_get(f"/api/admin/support/tickets/{TICKET_ID}", ADMIN_TOKEN)
    msg_list = data.get("messages", [])
    ws_found = [m for m in msg_list if "customer WS" in m.get("message","") or "Admin reply via WS" in m.get("message","")]
    test("WS messages saved to DB", len(ws_found) >= 2, f"{len(ws_found)} WS messages found")
except Exception as e:
    test("WS messages saved to DB", False, str(e))

# ==== TEST 5: Typing ====
print("\n" + "=" * 60)
print("TEST 5: Typing indicator")
print("=" * 60)
ws_a.send(json.dumps({"type": "typing"}))
time.sleep(0.3)
c_t = recv_event(ws_c, "typing")
test("Customer receives admin typing", c_t and c_t["sender_type"] == "admin",
     json.dumps(c_t) if c_t else "timeout")

ws_a.send(json.dumps({"type": "stop_typing"}))
time.sleep(0.3)
c_st = recv_event(ws_c, "stop_typing")
test("Customer receives stop_typing", c_st and c_st["sender_type"] == "admin")

# ==== TEST 6: Read receipt ====
print("\n" + "=" * 60)
print("TEST 6: Read receipt")
print("=" * 60)
ids = [m["id"] for m in msg_list[-3:]] if msg_list else [1,2,3]
ws_c.send(json.dumps({"type": "read", "message_ids": ids}))
time.sleep(0.3)
a_r = recv_event(ws_a, "read")
test("Admin receives read receipt", a_r and a_r["reader"] == "customer",
     f"ids={a_r.get('message_ids') if a_r else 'N/A'}")

# ==== TEST 7: Disconnect → offline ====
print("\n" + "=" * 60)
print("TEST 7: Disconnect → online:false")
print("=" * 60)
ws_c.close()
time.sleep(0.5)
a_off = recv_event(ws_a, "online")
test("Admin gets customer offline", a_off and a_off["online"]==False and a_off["sender_type"]=="customer",
     json.dumps(a_off) if a_off else "timeout")

# ==== TEST 8: Reconnect → online ====
print("\n" + "=" * 60)
print("TEST 8: Reconnect → online:true")
print("=" * 60)
ws_c2 = websocket.create_connection(f"{BASE}/api/support/ws?ticket_id={TICKET_ID}&license_key={LK}", sslopt=SSL_OPTS, timeout=5)
time.sleep(0.5)
a_on = recv_event(ws_a, "online")
test("Admin gets customer online", a_on and a_on["online"]==True and a_on["sender_type"]=="customer",
     json.dumps(a_on) if a_on else "timeout")

# ==== TEST 9: Ticket close ====
print("\n" + "=" * 60)
print("TEST 9: Admin close → ticket_closed event")
print("=" * 60)
try:
    api_put(f"/api/admin/support/tickets/{TICKET_ID}/status", {"status": "closed"}, ADMIN_TOKEN)
except Exception as e:
    test("API close ticket", False, str(e))
time.sleep(0.5)

c_cl = recv_event(ws_c2, "ticket_closed")
test("Customer receives ticket_closed", c_cl and c_cl["closed_by"]=="admin",
     json.dumps(c_cl) if c_cl else "timeout")

c_sm = recv_event(ws_c2, "message")
test("Customer receives system message", c_sm and c_sm["sender_type"]=="system",
     f"'{c_sm.get('text','')[:40]}'" if c_sm else "timeout")

a_cl = recv_event(ws_a, "ticket_closed")
test("Admin also receives ticket_closed", a_cl is not None)

# Cleanup
ws_c2.close(); ws_a.close()
try: api_put(f"/api/admin/support/tickets/{TICKET_ID}/status", {"status": "open"}, ADMIN_TOKEN)
except: pass

# ==== SUMMARY ====
print("\n" + "=" * 60)
p = sum(1 for _,v in results if v)
t = len(results)
print(f"RESULTS: {p}/{t} tests passed")
print("🎉 ALL TESTS PASSED!" if p==t else "⚠️ Failures: " + ", ".join(n for n,v in results if not v))
print("=" * 60)
