import {defineApplet} from "@akaia/applets-core";
import {DemoScreen} from "./applet";

export type AkaiaApplet = {
 init: () => void
}

export const VaultApplet: AkaiaApplet = defineApplet({
	routes: [{path: "/demo", component: DemoScreen}]
});
