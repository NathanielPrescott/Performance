import init, { greet } from "./service/pkg/service.js";

async function run() {
    await init();
    greet();
}

run();

async function perfMetricsREST(url) {
    let minTime = 10000000;
    let maxTime = 0;
    let totalTime = 0;
    let totalErrors = 0;
    let originalSize = 0;
    const totalCalls = document.getElementById("totalCalls").value;

    const textStart = performance.now();

    for (let i = 0; i < totalCalls; i++) {
        const start = performance.now();
        const response = await fetch(url);
        const size = (await response.blob()).size/1000/1000;

        if (originalSize === 0) {
            originalSize = size;
        }

        if (response.status !== 200 || originalSize !== size) {
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
        originalSize,
        totalErrors
    }
}

async function perfMetricsGRPC() {

}

async function getTextREST() {
    const metrics = await perfMetricsREST("http://127.0.0.1:8080/message");

    document.getElementById("minMaxTextREST").innerText = `Min Time: ${metrics.minTime} ms | Max Time: ${metrics.maxTime} ms`;
    document.getElementById("averageTextREST").innerText = `Average Time: ${metrics.averageTime} ms`;
    document.getElementById("overallTextREST").innerText = `Overall Time: ${metrics.overallTime} ms`;
    document.getElementById("sizeTextREST").innerText = `Size: ${metrics.originalSize} MB`;
    document.getElementById("errorsTextREST").innerText = `Errors: ${metrics.totalErrors}`;
}

async function getSmallImageREST() {
    const metrics = await perfMetricsREST("http://127.0.0.1:8080/image/request/Small");

    document.getElementById("minMaxSmallImageREST").innerText = `Min Time: ${metrics.minTime} ms | Max Time: ${metrics.maxTime} ms`;
    document.getElementById("averageSmallImageREST").innerText = `Average Time: ${metrics.averageTime} ms`;
    document.getElementById("overallSmallImageREST").innerText = `Overall Time: ${metrics.overallTime} ms`;
    document.getElementById("sizeSmallImageREST").innerText = `Size: ${metrics.originalSize} MB`;
    document.getElementById("errorsSmallImageREST").innerText = `Errors: ${metrics.totalErrors}`;
}

async function getMediumImageREST() {
    const metrics = await perfMetricsREST("http://127.0.0.1:8080/image/request/Medium");

    document.getElementById("minMaxMediumImageREST").innerText = `Min Time: ${metrics.minTime} ms | Max Time: ${metrics.maxTime} ms`;
    document.getElementById("averageMediumImageREST").innerText = `Average Time: ${metrics.averageTime} ms`;
    document.getElementById("overallMediumImageREST").innerText = `Overall Time: ${metrics.overallTime} ms`;
    document.getElementById("sizeMediumImageREST").innerText = `Size: ${metrics.originalSize} MB`;
    document.getElementById("errorsMediumImageREST").innerText = `Errors: ${metrics.totalErrors}`;
}

async function getLargeImageREST() {
    const metrics = await perfMetricsREST("http://127.0.0.1:8080/image/request/Large");

    document.getElementById("minMaxLargeImageREST").innerText = `Min Time: ${metrics.minTime} ms | Max Time: ${metrics.maxTime} ms`;
    document.getElementById("averageLargeImageREST").innerText = `Average Time: ${metrics.averageTime} ms`;
    document.getElementById("overallLargeImageREST").innerText = `Overall Time: ${metrics.overallTime} ms`;
    document.getElementById("sizeLargeImageREST").innerText = `Size: ${metrics.originalSize} MB`;
    document.getElementById("errorsLargeImageREST").innerText = `Errors: ${metrics.totalErrors}`;
}

async function getOriginalImageREST() {
    const metrics = await perfMetricsREST("http://127.0.0.1:8080/image/request/Original");

    document.getElementById("minMaxOriginalImageREST").innerText = `Min Time: ${metrics.minTime} ms | Max Time: ${metrics.maxTime} ms`;
    document.getElementById("averageOriginalImageREST").innerText = `Average Time: ${metrics.averageTime} ms`;
    document.getElementById("overallOriginalImageREST").innerText = `Overall Time: ${metrics.overallTime} ms`;
    document.getElementById("sizeOriginalImageREST").innerText = `Size: ${metrics.originalSize} MB`;
    document.getElementById("errorsOriginalImageREST").innerText = `Errors: ${metrics.totalErrors}`;
}

async function getTextGRPC() {

}

async function getSmallImageGRPC() {

}

async function getMediumImageGRPC() {

}

async function getLargeImageGRPC() {

}

async function getOriginalImageGRPC() {

}

document.addEventListener("DOMContentLoaded", () => {
    document.getElementById("runTests").addEventListener("click", async () => {
        console.log("Starting tests...");

        await getTextREST(); // REST call to get Text
        await getSmallImageREST(); // REST call to get Small Image
        await getMediumImageREST(); // REST call to get Medium Image
        await getLargeImageREST(); // REST call to get Large Image
        await getOriginalImageREST(); // REST call to get Original Image

        await getTextGRPC(); // gRPC call to get Text
        await getSmallImageGRPC(); // gRPC call to get Small Image
        await getMediumImageGRPC(); // gRPC call to get Medium Image
        await getLargeImageGRPC(); // gRPC call to get Large Image
        await getOriginalImageGRPC(); // gRPC call to get Original Image

        console.log("...tests completed!");
    });
});
