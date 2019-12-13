create table `hello`(
  id int not null auto_increment,
  name varchar(64) not null default '',
  primary key(id),
  key `idx_id`(id)
);

select * from hello;

select name as n from helloworld where id = ? and name like 'hello%';