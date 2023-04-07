// 导入所需的模块
const { ApiPromise, WsProvider } = require('@polkadot/api');

// 创建异步函数 main
async function main() {
    // 初始化连接到本地节点的提供程序
    const provider = new WsProvider('ws://127.0.0.1:9944');

    // 创建 API 实例并等待准备就绪
    const api = await ApiPromise.create({ provider });

    // 通过存储订阅系统事件
    api.query.system.events((events) => {
        // 在控制台上输出收到的事件数量
        console.log(`\n收到 ${events.length} 个事件：`);

        // 循环遍历每个事件并提取其阶段、事件和事件类型
        events.forEach((record) => {
            const { event, phase } = record;
            const types = event.typeDef;

            // 在控制台上输出当前正在处理的事件信息
            console.log(`\t${event.section}:${event.method}:: (phase=${phase.toString()})`);
            console.log(`\t\t${event.meta.documentation}`);

            // 循环遍历每个事件参数并显示其类型和数据
            event.data.forEach((data, index) => {
                console.log(`\t\t\t${types[index].type}: ${data.toString()}`);
            });
        });
    });
}

// 调用 main 函数并处理错误
main().catch((error) => {
    console.error(error);
    process.exit(-1);
});