CREATE INDEX idx_group_members ON group_members(group_id);
CREATE INDEX idx_group_users ON group_members(user_id);
CREATE INDEX idx_wishlist_group ON wishlist_items(group_id);
CREATE INDEX idx_wishlist_users ON wishlist_items(user_id);
CREATE INDEX idx_item_claims_item ON item_claims(item_id);
CREATE INDEX idx_item_claims_claimed ON item_claims(claimed_by);