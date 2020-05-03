import * as BABYLON from 'babylonjs';
import VertexBuffer = BABYLON.VertexBuffer;
import Mesh = BABYLON.Mesh;

function meshToBvh(mesh: Mesh): void {

    const indices = mesh.getIndices();
    if (!indices) {
        throw new Error("No indices in mesh.");
    }
    const positions = mesh.getVerticesData(VertexBuffer.PositionKind, false, false);
    if (!positions) {
        throw new Error("No positions in mesh.");
    }

    /*
    // If we need to transform to world coordinates.
    let index: number;
    let tri: number;
    let pt: number;
    
    const worldPositions = [];
    const wm = mesh.computeWorldMatrix(false);

    var transformed = Vector3.Zero();
    var position = Vector3.Zero();
    for (pt = 0; pt < positions.length; pt += 3) {
        Vector3.FromArrayToRef(positions, pt, position);
        Vector3.TransformCoordinatesToRef(position, wm, transformed);
        worldPositions.push(transformed.x, transformed.y, transformed.z);
    }
    */

}
