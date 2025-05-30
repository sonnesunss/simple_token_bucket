# 令牌桶算法

用来控制网络数据包、请求速率限制等.


## 工作原理

假设有一个桶，最多能装capacity个令牌，系统以固定速率向桶内添加令牌，当某个请求或数据包到来时，从桶中取令牌，例如1个，有两种细分情况:

    + 如果桶中有足够的令牌，就移除令牌，允许访问
    + 如果桶内没有足够的令牌，则拒绝请求或等待令牌补充

通过在添加上施加一个限制， 例如添加定时器以固定的速率添加令牌，可以实现控制整体的访问速率

此外它还允许突发的流量，在桶内有足够的令牌时，并不会仅限于固定


## 令牌桶实现

显而易见的一个定义是

```rust
pub struct TokenBucket {
    capacity: u32,        // 桶总容量
    tokens: f64,          // 当前令牌数量
    refill_rate: f64,     // 每秒生成多少个令牌
    last_refill_timestamp: Instant,     // 最后添加令牌到桶内的时间戳，使用单调时间类型
}
```

时间类型的选择上要注意，不能使用time类型，需要使用Instant，防止因为机器上面的时间调整影响到令牌添加的失败. 假如当前机器上面的时间被往前调整了就会添加失败.

那么它应该有哪些方法呢，上面的描述可知:

    + refill方法，即向桶内添加令牌的方法
    + try_consume方法，即消费桶内令牌的方法


在refill的实现中，需要定义几个策略:

    1. 令牌添加的策略，如何根据时间添加添加令牌，添加多少个令牌
    2. 是否在内部维护一个计时器，根据这个计时器产生的时间添加令牌，还是不需要，而是转而需要外部实现何时refiil的策略


try_consume的方法实现中，需要考虑如何消费令牌，以及在令牌桶为空时对后续而来的请求如何对待

丢弃还是放入队列内等待?

