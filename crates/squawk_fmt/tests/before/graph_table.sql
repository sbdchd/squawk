select  *  from  GRAPH_TABLE ( g MATCH (a) COLUMNS (a) );

select * from lateral GRAPH_TABLE(public.g MATCH (a IS person)-[e IS knows WHERE e.weight>1]->(b) WHERE a.active COLUMNS(a.name AS person_name,b.name)) AS matches;

select * from GRAPH_TABLE(g MATCH (a)<-[left_edge]-(b), (c)-[any_edge]-(d), (e)<-(f), (h)->(i), (j)-(k), ((x)->(y) WHERE x.active) COLUMNS(a));

select * from GRAPH_TABLE(g MATCH (a)->{1}(b), (c)-{,3}(d), (e)-{2,4}(f) COLUMNS(a));

select * from GRAPH_TABLE(a_very_long_property_graph_name MATCH (a_very_long_vertex_variable IS a_very_long_vertex_label WHERE a_very_long_vertex_filter_expression)-[a_very_long_edge_variable IS a_very_long_edge_label WHERE a_very_long_edge_filter_expression]->(a_second_very_long_vertex_variable IS a_second_very_long_vertex_label), (a_third_very_long_vertex_variable)-[a_second_very_long_edge_variable]-(a_fourth_very_long_vertex_variable) WHERE a_very_long_graph_filter_expression COLUMNS (a_very_long_vertex_variable.long_property_name AS first_long_output_column_name, a_second_very_long_vertex_variable.other_long_property_name AS second_output_column_name));

select * from
  /* before graph table */ GRAPH_TABLE /* before outer opening paren */ (
    /* before graph */ public /* before graph dot */ . /* before graph name */ g
    /* before match */ MATCH
    /* before first vertex opening paren */ ( /* before first variable */ a /* before is */ IS /* before first label */ person /* before vertex where */ WHERE /* before vertex expression */ a.active /* before first vertex closing paren */ )
    /* before first edge minus */ - /* before edge opening bracket */ [ /* before edge variable */ e /* before edge is */ IS /* before edge label */ knows /* before edge where */ WHERE /* before edge expression */ e.active /* before edge closing bracket */ ] /* before edge ending minus */ - /* before right angle */ >
    /* before second vertex */ (b)
    /* before qualifier opening curly */ { /* before qualifier min */ 1 /* before qualifier comma */, /* before qualifier max */ 3 /* before qualifier closing curly */ }
    /* before pattern comma */, /* before second pattern */
    /* before left angle */ < /* before left minus */ - /* before left opening bracket */ [left_edge /* before left closing bracket */ ] /* before left ending minus */ - (c),
    (d) /* before any edge */ - /* before any opening bracket */ [any_edge /* before any closing bracket */ ] /* before any ending minus */ - (e),
    /* before nested opening paren */ ( /* before nested pattern */ (x) /* before simple edge */ - /* before simple right angle */ > (y) /* before nested where */ WHERE /* before nested expression */ x.active /* before nested closing paren */ )
    /* before graph where */ WHERE /* before graph expression */ b.active
    /* before columns */ COLUMNS /* before columns opening paren */ (
      /* before first column */ a.name /* before column as */ AS /* before column name */ source /* before column comma */,
      /* before second column */ b.name
      /* before columns closing paren */ )
    /* before outer closing paren */
  ) /* after graph table */ AS /* before alias */ result;
