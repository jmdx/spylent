create table points (
  id serial primary key,
  tag varchar(100) not null,
  created bigint not null default extract(epoch from now()),
  value float not null
);