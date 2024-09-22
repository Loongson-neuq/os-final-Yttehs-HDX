## Vector 原理

`std::vector` 是在堆（heap）上的数组，创建时大小为0，可以使用 **resize()** 函数设置大小。

> 不是很确定，按照自己的理解大概解释一下。

使用 **push_back()** 函数可以向 vector 末尾添加元素，当元素数量超过 vector 的给定大小时，会使用 brk 系统调用申请更多的空间，用于存放新元素。

## 解释代码

```c
// 1
vector<int> vec1;
// 2
vector<int>* vec2 = new vector<int>();
```

1. vec1 变量是局部变量，在函数 **栈** 上创建。

2. vec2 是指针类型，由于 new 关键字，指向的内容在 **堆** 上分配。

vector 的数据总是在 **堆** 上。
