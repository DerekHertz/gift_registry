CREATE TABLE item_claims (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    item_id UUID REFERENCES wishlist_items(id) ON DELETE CASCADE,
    claimed_by UUID REFERENCES users(id) ON DELETE CASCADE,
    purchased BOOLEAN DEFAULT FALSE,
    claimed_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    -- multiple claims for expensive items
);