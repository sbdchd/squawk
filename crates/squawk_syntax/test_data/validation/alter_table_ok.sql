-- regression test for https://github.com/sbdchd/squawk/issues/597
alter table public.widget
  add constraint widget_config_schema_check check (
    checks.is_widget_config_valid('widget'::custom_types.widget_schema_type, config)
  ) not valid;

alter table public.widget_instance
  add constraint widget_instance_config_overrides_schema_check check (
    checks.is_widget_config_valid('widget_instance'::custom_types.widget_schema_type, config_overrides)
  ) not valid
