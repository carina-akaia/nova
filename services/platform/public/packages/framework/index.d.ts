import { AttributifyAttributes } from '@unocss/preset-attributify';
import * as class_variance_authority_types from 'class-variance-authority/types';
import { Component } from 'solid-js';
import { Button as Button$1 } from '@kobalte/core';
import { VariantProps } from 'class-variance-authority';

declare const buttonVariants: (props?: ({
    variant?: "default" | "destructive" | "outline" | "secondary" | "ghost" | "link" | null | undefined;
    size?: "default" | "sm" | "lg" | "icon" | null | undefined;
} & class_variance_authority_types.ClassProp) | undefined) => string;
type ButtonProps = Button$1.ButtonRootProps & VariantProps<typeof buttonVariants> & {};
declare const Button: Component<ButtonProps>;

declare module "solid-js" {
    namespace JSX {
        interface HTMLAttributes<T> extends AttributifyAttributes {
        }
    }
}

declare const init: () => void;

export { Button, type ButtonProps, buttonVariants, init };
