import { render, insert, createComponent, mergeProps, template } from 'solid-js/web';
import { splitProps } from 'solid-js';
import { Button as Button$1 } from '@kobalte/core';
import { cva } from 'class-variance-authority';
import { clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

// src/features/launcher/ui.tsx
function cn(...inputs) {
  return twMerge(clsx(inputs));
}

// src/common/ui/components/atoms/button.tsx
var buttonVariants = cva("inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50", {
  variants: {
    variant: {
      default: "bg-primary text-primary-foreground hover:bg-primary/90",
      destructive: "bg-destructive text-destructive-foreground hover:bg-destructive/90",
      outline: "border border-input hover:bg-accent hover:text-accent-foreground",
      secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
      ghost: "hover:bg-accent hover:text-accent-foreground",
      link: "text-primary underline-offset-4 hover:underline"
    },
    size: {
      default: "h-10 px-4 py-2",
      sm: "h-9 rounded-md px-3",
      lg: "h-11 rounded-md px-8",
      icon: "size-10"
    }
  },
  defaultVariants: {
    variant: "default",
    size: "default"
  }
});
var Button = (props) => {
  const [, rest] = splitProps(props, ["variant", "size", "class"]);
  return createComponent(Button$1.Root, mergeProps({
    get ["class"]() {
      return cn(buttonVariants({
        variant: props.variant,
        size: props.size
      }), props.class);
    }
  }, rest));
};
var _tmpl$ = /* @__PURE__ */ template(`<div un-w=full><h1>`);
var tagName = "akaia-applet-launcher";
var AkaiaAppletLauncher = class extends HTMLElement {
  static get observedAttributes() {
    return ["account_id", "applet_id", "route"];
  }
  attributeChangedCallback(name, oldValue, newValue) {
    console.log(`Attribute ${name} has changed to ${newValue}.`);
  }
  connectedCallback() {
    render(() => {
      const _self$ = this;
      return (() => {
        var _el$3 = _tmpl$(), _el$4 = _el$3.firstChild;
        insert(_el$4, () => `Loading ${_self$.attributes.getNamedItem("account_id")?.value}'s ` + _self$.attributes.getNamedItem("applet_id")?.value + ` ${_self$.attributes.getNamedItem("route")?.value}...`);
        insert(_el$3, createComponent(Button, {
          variant: "default",
          children: "Ok"
        }), null);
        return _el$3;
      })();
    }, this.attachShadow({
      mode: "closed"
    }).appendChild(document.createElement("main")));
  }
};
var launcherInit = () => {
  if (customElements.get(tagName) === void 0) customElements.define(tagName, AkaiaAppletLauncher);
};

// src/index.tsx
var init = () => {
  launcherInit();
};
init();

export { init };
