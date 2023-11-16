# Spark Query Plan

The org.apache.spark.sql.catalyst.plans package in Spark SQL's Catalyst module contains classes and traits for representing both logical and physical query plans. The main class relationships in this package are as follows:

- `QueryPlan`: This is an abstract base class for all query plans, both logical and physical. It provides common methods for manipulating and analyzing query plans, such as transform, foreach, collect, etc. It also defines the output method, which is used to get the output schema of a query plan.

- `LogicalPlan`: This is an abstract base class for logical query plans. It extends QueryPlan and provides additional methods for logical query plan manipulation and analysis.

- `PhysicalPlan`: This is an abstract base class for physical query plans. It extends QueryPlan and provides additional methods for physical query plan manipulation and execution.

- `SparkPlan`: This is an abstract base class for physical query plans in Spark SQL. It extends PhysicalPlan and provides methods for executing the query plan and computing the result.

1. What are the methods that each class adds on to QueryPlan?

```
The QueryPlan class provides a set of methods that are common to both logical and physical plans. These include methods for transforming and analyzing the plan, such as transform, transformExpressions, collect, foreach, find, exists, resolveOperators, resolveExpressions, and others. It also includes methods for computing statistics and output attributes.

The LogicalPlan class extends QueryPlan and adds methods that are specific to logical plans. These include methods for resolving and checking the plan, such as resolve, resolveChildren, resolveOperatorsUp, resolveOperatorsDown, analyzed, checkInputDataTypes, and others.

The PhysicalPlan class extends QueryPlan and adds methods that are specific to physical plans. These include methods for executing the plan, such as execute, executeCollect, executeTake, executeTail, and others. It also includes methods for computing metrics and build-side data size.

The SparkPlan class extends PhysicalPlan and adds methods that are specific to Spark's physical plans. These include methods for preparing and executing the plan in a Spark environment, such as prepare, doExecute, doExecuteBroadcast, and others. It also includes methods for managing subqueries and codegen stages.
```


2. Why does it need SparkPlan and PhysicalPlan separately?

```
The separation of PhysicalPlan and SparkPlan allows for a clear distinction between the general concept of a physical plan and the specifics of executing a physical plan in a Spark environment.

The PhysicalPlan class defines the basic structure and functionality of a physical plan, which includes the physical operators and methods for executing the plan. This class is not tied to any specific execution environment and can be used as a base class for physical plans in different systems.

The SparkPlan class, on the other hand, is specifically designed for executing physical plans in a Spark environment. It extends PhysicalPlan and adds methods that are specific to Spark, such as methods for preparing the plan for execution, executing the plan using Spark's execution engine, managing subqueries and codegen stages, and others.

This separation allows for better modularity and extensibility. If you want to implement a new execution environment for Spark SQL, you can create a new class that extends PhysicalPlan and implements the methods for your specific environment, without having to modify the existing SparkPlan class.
```


