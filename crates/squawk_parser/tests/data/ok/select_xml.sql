-- xmltable
select *
from xmltable(
    xmlnamespaces('http://example.com/books' as bk),
    '/bk:library/bk:book'
    passing xmlparse(document '<library xmlns="http://example.com/books">
               <book id="1">
                 <title>the great gatsby</title>
                 <author>f. scott fitzgerald</author>
                 <year>1925</year>
                 <price>12.99</price>
               </book>
               <book id="2">
                 <title>1984</title>
                 <author>george orwell</author>
                 <year>1949</year>
                 <price>14.99</price>
               </book>
               <book id="3">
                 <title>to kill a mockingbird</title>
                 <author>harper lee</author>
                 <year>1960</year>
                 <price>13.99</price>
               </book>
             </library>')
    columns
        row_num for ordinality,
        book_id integer path '@id',
        title text path 'bk:title',
        author text path 'bk:author',
        year integer path 'bk:year',
        price numeric(5,2) path 'bk:price' default 0.00,
        discount numeric(5,2) default 0.00 null
);
