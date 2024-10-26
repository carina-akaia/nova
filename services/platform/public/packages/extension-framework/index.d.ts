import { AttributifyAttributes } from '@unocss/preset-attributify';

declare module "solid-js" {
    namespace JSX {
        interface HTMLAttributes<T> extends AttributifyAttributes {
        }
    }
}
declare const init: () => void;

export { init };
