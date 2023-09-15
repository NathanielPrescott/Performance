async function perfMetrics(url) {
    let minTime = 10000000;
    let maxTime = 0;
    let totalTime = 0;
    let totalErrors = 0;
    const totalCalls = document.getElementById("totalCalls").value;

    const textStart = performance.now();

    for (let i = 0; i < totalCalls; i++) {
        const start = performance.now();
        const response = await fetch(url);

        if (response.status !== 200) {
            totalErrors++;
        } else {
            const end = performance.now();
            totalTime += (end - start);

            if (end - start < minTime) {
                minTime = end - start;
            }

            if (end - start > maxTime) {
                maxTime = end - start;
            }
        }

    }

    const overallTime = performance.now() - textStart;
    const averageTime = totalTime / totalCalls;

    return {
        minTime,
        maxTime,
        averageTime,
        overallTime,
        totalErrors
    }
}

async function getText() {
    const metrics = await perfMetrics("http://127.0.0.1:8080/image/deliver");

    document.getElementById("minMaxText").innerText = `Min Time: ${metrics.minTime} ms | Max Time: ${metrics.maxTime} ms`;
    document.getElementById("averageText").innerText = `Average Time: ${metrics.averageTime} ms`;
    document.getElementById("overallText").innerText = `Overall Time: ${metrics.overallTime} ms`;
    document.getElementById("errorsText").innerText = `Errors: ${metrics.totalErrors}`;
}

async function getSmallImage() {
    const metrics = await perfMetrics("http://127.0.0.1:8080/image/request/Small");

    document.getElementById("minMaxSmallImage").innerText = `Min Time: ${metrics.minTime} ms | Max Time: ${metrics.maxTime} ms`;
    document.getElementById("averageSmallImage").innerText = `Average Time: ${metrics.averageTime} ms`;
    document.getElementById("overallSmallImage").innerText = `Overall Time: ${metrics.overallTime} ms`;
    document.getElementById("errorsSmallImage").innerText = `Errors: ${metrics.totalErrors}`;
}

async function getMediumImage() {
    const metrics = await perfMetrics("http://127.0.0.1:8080/image/request/Medium");

    document.getElementById("minMaxMediumImage").innerText = `Min Time: ${metrics.minTime} ms | Max Time: ${metrics.maxTime} ms`;
    document.getElementById("averageMediumImage").innerText = `Average Time: ${metrics.averageTime} ms`;
    document.getElementById("overallMediumImage").innerText = `Overall Time: ${metrics.overallTime} ms`;
    document.getElementById("errorsMediumImage").innerText = `Errors: ${metrics.totalErrors}`;
}

async function getLargeImage() {
    const metrics = await perfMetrics("http://127.0.0.1:8080/image/request/Large");

    document.getElementById("minMaxLargeImage").innerText = `Min Time: ${metrics.minTime} ms | Max Time: ${metrics.maxTime} ms`;
    document.getElementById("averageLargeImage").innerText = `Average Time: ${metrics.averageTime} ms`;
    document.getElementById("overallLargeImage").innerText = `Overall Time: ${metrics.overallTime} ms`;
    document.getElementById("errorsLargeImage").innerText = `Errors: ${metrics.totalErrors}`;
}

async function getOriginalImage() {
    const metrics = await perfMetrics("http://127.0.0.1:8080/image/request/Original");

    document.getElementById("minMaxOriginalImage").innerText = `Min Time: ${metrics.minTime} ms | Max Time: ${metrics.maxTime} ms`;
    document.getElementById("averageOriginalImage").innerText = `Average Time: ${metrics.averageTime} ms`;
    document.getElementById("overallOriginalImage").innerText = `Overall Time: ${metrics.overallTime} ms`;
    document.getElementById("errorsOriginalImage").innerText = `Errors: ${metrics.totalErrors}`;
}

document.addEventListener("DOMContentLoaded", () => {
    document.getElementById("runTests").addEventListener("click", async () => {
        console.log("Starting tests...");

        await getText(); // REST call to get Text
        await getSmallImage(); // REST call to get Small Image
        await getMediumImage(); // REST call to get Medium Image
        await getLargeImage(); // REST call to get Large Image
        await getOriginalImage(); // REST call to get Original Image

        // gRPC call to get Text
        // gRPC call to get Small Image
        // gRPC call to get Medium Image
        // gRPC call to get Large Image
        // gRPC call to get Original Image

        console.log("...tests completed!");
    });
});
