# migrator

manage your migrations simple

if you have dir "migrations" in your project where you keep all migrations files numbered like

```
0001_create_new_table.sql
0002_add_column.sql
0003_change_column_name.sql
```

you can use the Migrator to automate migrations creation

to create a new migration simply call ```migrator --name='create table for users'```

imagine you have the listing shown above

what you will see is 

```
0001_create_new_table.sql
0002_add_column.sql
0003_change_column_name.sql
0004_create_table_for_users.sql
```

you can also sort your migrations very easily

Let's say you just have rebased the main branch of your project and you have a migration collision like:

```
0001_create_new_table.sql
0002_add_column.sql
0003_change_column_name.sql
0003_create_table_authors.sql
```

you simply call ```migrator --sort```

to have 

```
0001_create_new_table.sql
0002_add_column.sql
0003_change_column_name.sql
0004_create_table_authors.sql
```
