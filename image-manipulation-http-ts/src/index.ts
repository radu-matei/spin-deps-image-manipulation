import { ResponseBuilder } from "@fermyon/spin-sdk";
// We have to ignore here because `spin deps` does not yet generate type bindings for TS
//@ts-ignore
import { grayscale } from "component:image-manipulation-lib/image-manipulation"

export async function handler(req: Request, res: ResponseBuilder) {
    if (req.method !== "POST") {
        res.status(405)
        res.send("Method not allowed")
        return
    }
    try {
        let body = await req.arrayBuffer();
        let image = new Uint8Array(body);
        let bw = grayscale(image, 100);
        res.set({ "Content-Type": "image/jpeg" });
        res.send(bw);
    } catch (e) {
        console.error(e);
        res.status(500)
        res.send("Internal Server Error")
    }
}
