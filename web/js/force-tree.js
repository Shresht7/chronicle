import { getLatestSnapshotTree } from './api.js';

export async function drawTreeVisualization() {
    const treeData = await getLatestSnapshotTree();

    if (!treeData || !treeData.name) {
        d3.select("#tree-visualization").text("No file tree data available.");
        return;
    }

    d3.select("#tree-visualization").text("");

    const width = 960;
    const height = 600;

    const svg = d3.select("#tree-visualization")
        .append("svg")
        .attr("width", width)
        .attr("height", height);

    const g = svg.append("g");

    const zoomBehavior = d3.zoom()
        .scaleExtent([0.1, 10])
        .on("zoom", zoomedTree);

    svg.call(zoomBehavior);

    function zoomedTree(event) {
        g.attr("transform", event.transform);
    }

    const root = d3.hierarchy(treeData);

    const simulation = d3.forceSimulation(root.descendants())
        .force("link", d3.forceLink(root.links()).id(d => d.id).distance(100))
        .force("charge", d3.forceManyBody().strength(-200))
        .force("center", d3.forceCenter(width / 2, height / 2));

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
        .attr("fill", d => d.children ? "#555" : "#999");

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
