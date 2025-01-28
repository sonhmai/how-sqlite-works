# Cache flush

``` 
Transaction Management
    Normal processing
        Read operation
        Write operation
        Cache flush -> we're here
        Commit operation
        Statement operations 
        Setting up savepoints
        Releasing savepoints
```

cache flush is an `internal` operation of the pager module.
Tree module cannot call it directly.

Two situations pager needs to flush a page out of page cache
1. cache has filled up and needs cache replacement
2. transaction ready to commit its change

