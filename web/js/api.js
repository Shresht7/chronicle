export async function getSnapshots() {
    try {
        const data = await d3.json("/api/snapshots");
        console.log("Fetched timeline data:", data);
        return data;
    } catch (error) {
        console.error("Error fetching timeline data:", error);
        d3.select("#visualization").text("Error loading timeline data.");
        return null;
    }
}

export async function getLatestSnapshotTree() {
    try {
        const treeData = await d3.json("/api/latest_snapshot_tree");
        console.log("Fetched tree data:", treeData);
        return treeData;
    } catch (error) {
        console.error("Error fetching tree data:", error);
        return null;
    }
}
