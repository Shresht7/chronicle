console.log("main.js loaded");

// Dummy D3.js visualization (timeline example)
async function drawVisualization() {
    const data = await d3.json("/api/snapshots");
    console.log("Fetched data:", data);

    const svg = d3.select("#visualization")
        .append("svg")
        .attr("width", 800)
        .attr("height", 100);

    const xScale = d3.scaleTime()
        .domain([new Date(data[0].timestamp), new Date(data[data.length - 1].timestamp)])
        .range([50, 750]);

    svg.selectAll("circle")
        .data(data)
        .enter()
        .append("circle")
        .attr("cx", d => xScale(new Date(d.timestamp)))
        .attr("cy", 50)
        .attr("r", 5)
        .attr("fill", "steelblue");

    svg.append("g")
        .attr("transform", "translate(0, 50)")
        .call(d3.axisBottom(xScale));

    d3.select("#visualization").text(""); // Clear loading text
}

drawVisualization();