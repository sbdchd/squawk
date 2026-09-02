alter /* property */ property /* graph */ graph /* if */ if /* exists */ exists /* name */ public.social_graph /* action */ rename /* to */ to /* target */ archived_graph /* end */;

alter property graph graph_with_an_exceptionally_long_descriptive_name add vertex tables (customer_accounts key (customer_identifier) no properties, merchant_accounts key (merchant_identifier) properties all columns);

alter property graph social_graph /* first add */ add /* vertex tables */ vertex tables (people key (id)) /* second add */ add /* edge tables */ edge tables (follows /* source */ source /* source key */ key (follower_id) references people (id) /* destination */ destination /* destination key */ key (followed_id) references people (id));

alter property graph social_graph alter /* kind */ vertex /* table */ table /* element */ people alter /* label */ label /* name */ person add /* properties */ properties (full_name, birth_date as date_of_birth);

alter property graph social_graph alter relationship table follows alter label connection drop properties (created_at, source_system);

alter property graph social_graph alter vertex table people /* before alter */ alter label person add properties (id);

alter property graph social_graph alter vertex table people -- before alter
alter label person drop properties (id);

alter property graph social_graph add vertex tables (people key (id) label person properties (name) /* between labels */ label human properties (nickname));

alter property graph social_graph alter /* before vertex */ vertex /* before table */ table /* before element */ people /* before drop */ drop /* before label */ label /* before label name */ person;

alter property graph social_graph alter vertex -- before table
table people drop label person;

alter property graph social_graph alter relationship /* before table */ table follows alter label connection drop properties (created_at);

alter property graph social_graph drop vertex tables (people, organizations) cascade;

alter property graph social_graph owner /* to */ to /* role */ graph_administrator;
