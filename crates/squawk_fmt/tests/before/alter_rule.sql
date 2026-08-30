alter rule old_rule on public.orders rename to current_orders;

alter rule rule_with_a_very_long_descriptive_name on reporting.relation_with_a_very_long_descriptive_name rename to renamed_rule_with_a_very_long_descriptive_name;

alter /* before rule */ rule /* before old name */ commented_rule /* before on */ on /* before relation */ public.commented_orders /* before rename */ rename /* before to */ to /* before new name */ renamed_commented_rule /* before semicolon */;
