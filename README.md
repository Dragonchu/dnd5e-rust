# dnd5e-rust

dnd5e-rust是一个rust编写的dnd5e游戏引擎，不包括数据存储、网络传输协议。

dnd5e-rust定义的PO对象可以存储，engine根据输入的PO对象运行，并修改运行的BO对象，任何时间可以将引擎中的BO对象转化为PO对象并存储。