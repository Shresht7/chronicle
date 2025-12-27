console.log("main.js loaded");

// --- Timeline Visualization ---
async function drawTimelineVisualization() {
    let data;
    try {
        data = await d3.json("/api/snapshots");
        console.log("Fetched timeline data:", data);
    } catch (error) {
        console.error("Error fetching timeline data:", error);
        d3.select("#visualization").text("Error loading timeline data.");
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
        console.error("Invalid date domain for timeline visualization:", minDate, maxDate);
        container.text("Insufficient or invalid date data for timeline visualization.");
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

// --- Force-Directed Tree Graph Visualization ---
async function drawTreeVisualization() {
    let treeData;
    try {
        treeData = await d3.json("/api/latest_snapshot_tree");
        console.log("Fetched tree data:", treeData);
    } catch (error) {
        console.error("Error fetching tree data:", error);
        d3.select("#tree-visualization").text("Error loading file tree data.");
        return;
    }

    if (!treeData || !treeData.name) { // Check if root node exists
        d3.select("#tree-visualization").text("No file tree data available.");
        return;
    }

    d3.select("#tree-visualization").text(""); // Clear initial loading text

    const width = 960;
    const height = 600;

    const svg = d3.select("#tree-visualization")
        .append("svg")
        .attr("width", width)
        .attr("height", height);

    const g = svg.append("g"); // Main group to apply transforms

    // Add zoom behavior to the SVG
    const zoomBehavior = d3.zoom()
        .scaleExtent([0.1, 10]) // Adjust scale extent as needed
        .on("zoom", zoomedTree);

    svg.call(zoomBehavior);

    function zoomedTree(event) {
        g.attr("transform", event.transform);
    }

    // Create the hierarchy
    const root = d3.hierarchy(treeData);

    // Create a simulation for the forces
    const simulation = d3.forceSimulation(root.descendants())
        .force("link", d3.forceLink(root.links()).id(d => d.id).distance(100))
        .force("charge", d3.forceManyBody().strength(-200)) // Repel nodes
        .force("center", d3.forceCenter(width / 2, height / 2)); // Keep centered within the SVG, not the `g`

    const link = g.append("g")
        .attr("stroke", "#999")
        .attr("stroke-opacity", 0.6)
        .selectAll("line")
        .data(root.links())
        .join("line");

    const node = g.append("g")
        .attr("stroke", "#fff")
        .attr("stroke-width", 1.5)
        .selectAll("circle")
        .data(root.descendants())
        .join("circle")
        .attr("r", 5)
        .attr("fill", d => d.children ? "#555" : "#999"); // Directories darker, files lighter

    const label = g.append("g")
        .attr("class", "labels")
        .selectAll("text")
        .data(root.descendants())
        .enter().append("text")
        .attr("class", "node-label")
        .text(d => d.data.name);

    simulation.on("tick", () => {
        link
            .attr("x1", d => d.source.x)
            .attr("y1", d => d.source.y)
            .attr("x2", d => d.target.x)
            .attr("y2", d => d.target.y);

        node
            .attr("cx", d => d.x)
            .attr("cy", d => d.y);

        label
            .attr("x", d => d.x + 8)
            .attr("y", d => d.y + 3);
    });

    // Add drag functionality to nodes
    node.call(d3.drag()
        .on("start", dragstarted)
        .on("drag", dragged)
        .on("end", dragended));

    function dragstarted(event) {
        if (!event.active) simulation.alphaTarget(0.3).restart();
        event.subject.fx = event.subject.x;
        event.subject.fy = event.subject.y;
    }

    function dragged(event) {
        event.subject.fx = event.x;
        event.subject.fy = event.y;
    }

    function dragended(event) {
        if (!event.active) simulation.alphaTarget(0);
        event.subject.fx = null;
        event.subject.fy = null;
    }
}

// --- Radial Tree Graph Visualization ---
const defaultRadialGraphOptions = {
    // SVG options
    marginTop: 10,
    marginRight: 10,
    marginBottom: 10,
    marginLeft: 10,

    // Radial Tree Options
    angle: 2 * Math.PI, // 360 degrees sweep
    radius: 800, // Adjusted for typical screen size
    separator: (a, b) => a.parent === b.parent ? 1 : 2, // 1 for siblings, 2 for non-siblings
    sortFn: (a, b) => d3.ascending(a.data.name, b.data.name), // sort by name in ascending order

    // Styling Options - Links
    linkStroke: '#333',
    linkStrokeOpacity: 0.6,
    linkStrokeLineCap: 'round',
    linkStrokeLineJoin: 'round',
    linkStrokeWidth: 1.5,

    nodeFill: '#fff',
    nodeRadius: 2.5,
    nodeTextStroke: "#eee",
    nodeTextStrokeWidth: 2,
    nodeTextSize: 10,
};

async function drawRadialTreeVisualization() {
    let treeData;
    try {
        treeData = await d3.json("/api/latest_snapshot_tree");
        console.log("Fetched radial tree data:", treeData);
    } catch (error) {
        console.error("Error fetching radial tree data:", error);
        d3.select("#radial-tree-visualization").text("Error loading radial tree data.");
        return;
    }

    if (!treeData || !treeData.name) {
        d3.select("#radial-tree-visualization").text("No radial tree data available.");
        return;
    }

    d3.select("#radial-tree-visualization").text(""); // Clear initial loading text

    const options = { ...defaultRadialGraphOptions }; // Use default options

    const width = (2 * options.radius) + options.marginLeft + options.marginRight;
    const height = (2 * options.radius) + options.marginTop + options.marginBottom;


    const svg = d3.select("#radial-tree-visualization").append("svg")
        .attr("width", width)
        .attr("height", height)
        .attr("viewBox", [
            -width / 2, // Centered viewbox
            -height / 2,
            width,
            height
        ]);

    const g = svg.append("g"); // Group for tree elements

    // Create tree
    const tree = d3.tree()
        .size([options.angle, options.radius])
        .separation(options.separator);

    // Create node tree
    const root = tree(d3.hierarchy(treeData)
        .sort(options.sortFn)); // Sort hierarchy directly

    // Create descendants and links
    const descendants = root.descendants();
    const links = root.links();

    // Create links
    g.append("g")
        .attr("fill", "none")
        .attr("stroke", options.linkStroke)
        .attr("stroke-width", options.linkStrokeWidth)
        .attr("stroke-opacity", options.linkStrokeOpacity)
        .attr("stroke-linecap", options.linkStrokeLineCap)
        .attr("stroke-linejoin", options.linkStrokeLineJoin)
        .selectAll("path")
        .data(links)
        .join("path")
        .attr("d", d3.linkRadial().angle(d => d.x).radius(d => d.y));

    // Create nodes
    const node = g.append("g")
        .selectAll("a")
        .data(descendants)
        .join("a")
        .attr("transform", d => `rotate(${d.x * 180 / Math.PI - 90}) translate(${d.y},0)`);

    // Add node circle
    node.append("circle")
        .attr("fill", options.nodeFill)
        .attr("r", options.nodeRadius);

    // Add title to the nodes
    node.append('title')
        .text(d => d.data.name);

    // Add node text
    node.append("text")
        .attr("transform", d => `rotate(${d.x >= Math.PI ? 180 : 0})`)
        .attr("dy", "0.32em")
        .attr("x", d => d.x < Math.PI === !d.children ? 6 : -6)
        .attr("text-anchor", d => d.x < Math.PI === !d.children ? "start" : "end")
        .attr("paint-order", "stroke")
        .attr("stroke", options.nodeTextStroke)
        .attr("stroke-width", options.nodeTextStrokeWidth)
        .attr("font-size", options.nodeTextSize)
        .attr('fill', options.nodeFill)
        .text(d => d.data.name); // Removed 'i' parameter as it's not used in this context

    // Add zoom/pan for radial tree
    const radialZoomBehavior = d3.zoom()
        .scaleExtent([0.1, 10]) // Adjust scale extent as needed
        .on("zoom", zoomedRadialTree);

    svg.call(radialZoomBehavior);

    function zoomedRadialTree(event) {
        g.attr("transform", event.transform);
    }
}


// Call both visualizations
drawTimelineVisualization();
drawTreeVisualization();
drawRadialTreeVisualization(); // Call the new radial tree visualization
