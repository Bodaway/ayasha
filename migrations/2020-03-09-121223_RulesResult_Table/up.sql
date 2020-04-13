-- Your SQL goes here
create table rules_result (
	rule_name text not null ,
	dt_execution Timestamp not null,
	success boolean not null,
	details text not null,
	PRIMARY KEY (rule_name,dt_execution)
);











