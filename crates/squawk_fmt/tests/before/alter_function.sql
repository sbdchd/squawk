ALTER /* function */ FUNCTION /* signature */ public.calculate_a_very_long_and_descriptive_result(/* type */ integer, /* type */ text) /* action */ SECURITY /* definer */ DEFINER /* restrict */ RESTRICT;

ALTER FUNCTION public.f(integer) /* action */ SET /* parameter */ work_mem /* to */ TO /* value */ '64MB';

ALTER FUNCTION public.f(integer) DEPENDS /* on */ ON /* extension */ EXTENSION /* name */ extension_name;

ALTER FUNCTION public.f(integer) NO /* depends */ DEPENDS /* on */ ON /* extension */ EXTENSION /* name */ extension_name;

ALTER FUNCTION public.f(integer) RENAME /* to */ TO /* name */ renamed_function;

ALTER FUNCTION public.f(integer) OWNER /* to */ TO /* role */ current_role;

ALTER FUNCTION public.f(integer) SET /* schema */ SCHEMA /* name */ archive;
