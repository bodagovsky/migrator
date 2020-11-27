# migrator
automating migration file creation and sorting.

to create a new migration run:
```migrator --name='create new table'``` 

migrator will create a migration with next serial number

"000x_create_new_table.sql"

To sort all the migrations in /migrations directory, provide --sort flag

```migrator --sort```


WARNING! if you do not have dir /migrations or it is empty, migrator will panic

