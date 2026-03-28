DO $$
DECLARE
    v_store_id INT := 1;
    t TEXT;
    synced_tables TEXT[] := ARRAY[
        'synced_return_items', 'synced_returns',
        'synced_input_invoice_items', 'synced_input_invoices',
        'synced_invoice_items', 'synced_invoice_payments', 'synced_invoices',
        'synced_purchase_items', 'synced_purchase_orders',
        'synced_inventory_counters',
        'synced_product_transactions', 'synced_product_batches', 'synced_product_units', 'synced_products',
        'synced_customer_transactions', 'synced_supplier_transactions', 'synced_cash_transactions', 'synced_payment_vouchers',
        'synced_customers', 'synced_suppliers',
        'synced_promotions', 'synced_vouchers', 'synced_daily_closings', 'synced_store_funds',
        'synced_loyalty_transactions_v2', 'synced_store_settings'
    ];
BEGIN
    RAISE NOTICE 'Bắt đầu dọn dẹp dữ liệu Zero State cho store_id (%)', v_store_id;

    FOREACH t IN ARRAY synced_tables
    LOOP
        EXECUTE format('DELETE FROM %I WHERE store_id = %L', t, v_store_id);
    END LOOP;

    DELETE FROM sync_journal WHERE store_id = v_store_id;
    DELETE FROM sync_inbox WHERE store_id = v_store_id;
    DELETE FROM sync_conflicts WHERE store_id = v_store_id;
    
    UPDATE sync_devices SET pull_cursor = 0, last_push_at = NULL, last_pull_at = NULL WHERE store_id = v_store_id;

    RAISE NOTICE 'Hoàn tất! Cửa hàng % đã trở về môi trường Zero State rỗng hoàn toàn.', v_store_id;
END $$;
