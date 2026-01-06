import { getLatestSnapshotTree } from './api.js';

const defaultRadialGraphOptions = {
    marginTop: 10,
    marginRight: 10,
    marginBottom: 10,
    marginLeft: 10,
    angle: 2 * Math.PI,
    radius: 800,
    separator: (a, b) => a.parent === b.parent ? 1 : 2,
    sortFn: (a, b) => d3.ascending(a.data.name, b.data.name),
    linkStroke: '#333',
    linkStrokeOpacity: 0.6,
    linkStrokeLineCap: 'round',
    linkStrokeLineJoin: 'round',
    linkStrokeWidth: 1.5,
    nodeFill: '#333',
    nodeRadius: 2.5,
    nodeTextStroke: "#333",
    nodeTextStrokeWidth: 0,
    nodeTextSize: 10,
};

export async function drawRadialTreeVisualization() {
    const treeData = await getLatestSnapshotTree();

    if (!treeData || !treeData.name) {
        d3.select("#radial-tree-visualization").text("No radial tree data available.");
        return;
    }

    d3.select("#radial-tree-visualization").text("");

    const options = { ...defaultRadialGraphOptions };

    const svgWidth = 960;
    const svgHeight = 600;

    const svg = d3.select("#radial-tree-visualization").append("svg")
        .attr("width", svgWidth)
        .attr("height", svgHeight);

    const g = svg.append("g");

    const tree = d3.tree()
        .size([options.angle, options.radius])
        .separation(options.separator);

    const root = tree(d3.hierarchy(treeData).sort(options.sortFn));
    const descendants = root.descendants();
    const links = root.links();

    const max_y = d3.max(descendants, d => d.y);
    const scale = Math.min(1, Math.min(svgWidth / (2 * max_y), svgHeight / (2 * max_y)) * 0.9);
    const initialTransform = d3.zoomIdentity
        .translate(svgWidth / 2, svgHeight / 2)
        .scale(scale);

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

    const node = g.append("g")
        .selectAll("a")
        .data(descendants)
        .join("a")
        .attr("transform", d => `rotate(${d.x * 180 / Math.PI - 90}) translate(${d.y},0)`);

    node.append("circle")
        .attr("fill", options.nodeFill)
        .attr("r", options.nodeRadius);

    node.append('title')
        .text(d => d.data.name);

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
        .text(d => d.data.name);

    const radialZoomBehavior = d3.zoom()
        .scaleExtent([0.1, 10])
        .on("zoom", zoomedRadialTree);

    svg.call(radialZoomBehavior);
    svg.call(radialZoomBehavior.transform, initialTransform);

    function zoomedRadialTree(event) {
        g.attr("transform", event.transform);
    }
}
