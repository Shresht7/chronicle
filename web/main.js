console.log("main.js loaded");

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

    d3.select("#visualization").text(""); // Clear initial loading text

    // Define dimensions and margins
    const margin = { top: 20, right: 30, bottom: 30, left: 40 };
    const width = 960 - margin.left - margin.right;
    const height = 150 - margin.top - margin.bottom;

    const container = d3.select("#visualization");

    const svg = container.append("svg")
        .attr("width", width + margin.left + margin.right)
        .attr("height", height + margin.top + margin.bottom)
        .append("g")
        .attr("transform", `translate(${margin.left},${margin.top})`);

    // Parse dates
    data.forEach(d => {
        d.parsedTimestamp = new Date(d.timestamp);
    });

    const minDate = d3.min(data, d => d.parsedTimestamp);
    const maxDate = d3.max(data, d => d.parsedTimestamp);

    if (isNaN(minDate) || isNaN(maxDate) || minDate === maxDate) {
        console.error("Invalid date domain for visualization:", minDate, maxDate);
        container.text("Insufficient or invalid date data for visualization.");
        return;
    }

    // Initial X scale
    const xScale = d3.scaleTime()
        .domain([minDate, maxDate])
        .range([0, width]);

    // Create a group for the x-axis
    const xAxis = svg.append("g")
        .attr("transform", `translate(0,${height})`)
        .call(d3.axisBottom(xScale));

    // Create a group for the circles (snapshots)
    const circlesGroup = svg.append("g");

    // Draw circles initially
    circlesGroup.selectAll("circle")
        .data(data)
        .enter()
        .append("circle")
        .attr("cx", d => xScale(d.parsedTimestamp))
        .attr("cy", height / 2)
        .attr("r", 5)
        .attr("fill", "steelblue");

    // Add zoom behavior
    const zoom = d3.zoom()
        .scaleExtent([1, 100]) // Increased maximum zoom level
        .translateExtent([[0, 0], [width, height]]) // Prevent panning outside limits
        .extent([[0, 0], [width, height]])
        .on("zoom", zoomed);

    // Apply zoom behavior to a transparent rectangle to capture events
    svg.append("rect")
        .attr("class", "zoom")
        .attr("width", width)
        .attr("height", height)
        .attr("fill", "none")
        .attr("pointer-events", "all")
        .call(zoom);

    function zoomed(event) {
        const newXScale = event.transform.rescaleX(xScale);
        xAxis.call(d3.axisBottom(newXScale));
        circlesGroup.selectAll("circle")
            .attr("cx", d => newXScale(d.parsedTimestamp));
    }
}

drawVisualization();