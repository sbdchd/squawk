create view foo as 
  select b.y from (( select y from bar )) as b
  order by y desc;
