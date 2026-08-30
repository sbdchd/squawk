drop operator class public.text_pattern_ops using btree;

drop operator class if exists extraordinarily_long_schema_name.extraordinarily_long_operator_class_name using extraordinarily_long_access_method_name cascade;

-- comments in every position
drop /* operator */ operator /* class */ class /* if */ if /* exists */ exists /* class name */ public /* dot */ . text_pattern_ops /* using */ using /* method */ btree /* behavior */ restrict /* end */;
