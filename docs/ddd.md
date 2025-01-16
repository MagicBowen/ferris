## Domain Driven Design

### 建模元素

领域驱动设计（Domain Driven Design: DDD）中把建模元素分为如下类型，并给出了实现的最佳实践；

- Event：一个组件从外部接受消息，一般为纯数据类型，可以被序列化和反序列化。
- Service：服务类，对外提供接口和服务；作为组件的入口，一般接收 Event 作为输入，调用内部处理逻辑，然后返回 Event。
- Aggregate：聚合根，一组相关的对象，一般是有一致性约束的一组对象；作为一个整体进行操作；其中一个 Entity 作为 Root，有全局 ID；其它 Entity 不对外。
- Entity：实体类，一般是具有唯一标识的对象，有清晰的生命周期，使用 ID 比较是否相同。在 Aggregate 内部有局部 ID，否则有全局 ID；
- Value Object：值对象，一般是没有 ID 的对象，只有属性；所有属性相等则相等。一般作为 Entity 的属性，或者作为参数传递。一般 Value Object 是不可变的，整体替换（不绝对）；
- Repository：仓库，提供对 Aggregate 的跨 Service 存储，提供接口供 Service 获取、增加、移除 Aggregate；一般 Service 从接口参数或者 Event中获得 ID，然后通过 Repository 获取 Aggregate，调用其方法完成业务逻辑；
- Factory：工厂，提供创建 Aggregate 的方法，一般是复杂的创建逻辑；Factory 也是供 Service 调用的，创建的 Aggregate 通过 Repository 存储；

### 惯用实现

- Event：一般是纯数据类，作为 Service 的参数和返回值，入参一般是不可变的，内存由外部管理；
- Service：一般是无状态的，接收 Event，从 Repo 中查找并调用 Aggregate 的方法，返回 Event；Service 一般是无状态的，可以并发调用；
- Repository：一般是容器类，存储和查询 Aggregate，供 Service 使用；每种 Aggregate 一个 Repository；Repository一般是单例，并发访问需要加锁；Service 根据业务需要调用合适的 Repository；
- Factory：一般是创建 Aggregate 的方法，复杂的创建逻辑；Factory 一般是无状态的，大多是静态类和方法，可以并发调用；
- Aggregate：一般是有状态的，有一致性约束，内部有 Entity 和 Value Object；Aggregate 一般是有状态的，不可并发访问，需要加锁；有生命周期，由对应的 Factory 创建，保存在对应的 Repository 中；
- Entity：一般是有状态的，有 ID，有生命周期；Entity 一般是有状态的，不可并发访问（同步控制由上层的 Aggregate 控制）；有生命周期，由对应的 Aggregate 创建并聚合；Entity需要实现正确的 `eq` 和 `hash` 方法（借助 ID）；
- Value Object：一般是无状态的，不可变的；可以被多个 Entity 引用； 也可以直接被 Entity 组合；Value Object 需要实现正确的 `eq` 和 `hash` 方法（全属性）；
