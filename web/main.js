console.log("main.js loaded");

// Dummy D3.js visualization (timeline example)
async function drawVisualization() {
    let data;
    try {
        data = await d3.json("/api/snapshots");
        console.log("Fetched data:", data);
    } catch (error) {
        console.error("Error fetching data:", error);
        d3.select("#visualization").text("Error loading data.");
        return;
    }

    if (!data || data.length === 0) {
        d3.select("#visualization").text("No snapshot data available.");
        return;
    }

    // Clear initial loading text
    d3.select("#visualization").text("");

    const container = d3.select("#visualization");
    console.log("Visualization container:", container.node());

    const svg = container
        .append("svg")
        .attr("width", 800)
        .attr("height", 100);
    console.log("SVG element created:", svg.node());

    // Parse dates
    data.forEach(d => {
        d.parsedTimestamp = new Date(d.timestamp);
    });

    // Ensure valid domain for x-scale
    const minDate = d3.min(data, d => d.parsedTimestamp);
    const maxDate = d3.max(data, d => d.parsedTimestamp);

    if (isNaN(minDate) || isNaN(maxDate) || minDate === maxDate) {
        console.error("Invalid date domain for visualization:", minDate, maxDate);
        container.text("Insufficient or invalid date data for visualization.");
        return;
    }

    const xScale = d3.scaleTime()
        .domain([minDate, maxDate])
        .range([50, 750]);
    console.log("xScale domain:", xScale.domain());
    console.log("xScale range:", xScale.range());


    svg.selectAll("circle")
        .data(data)
        .enter()
        .append("circle")
        .attr("cx", d => xScale(d.parsedTimestamp))
        .attr("cy", 50)
        .attr("r", 5)
        .attr("fill", "steelblue");
    console.log("Circles appended:", svg.selectAll("circle").nodes().length);


    svg.append("g")
        .attr("transform", "translate(0, 50)")
        .call(d3.axisBottom(xScale));
    console.log("Axis appended.");
}

drawVisualization();