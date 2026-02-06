<script lang="ts">
  type Variant = 'elevated' | 'outline' | 'ghost';
  type Padding = 'sm' | 'md' | 'lg';

  export let variant: Variant = 'elevated';
  export let padding: Padding = 'md';
  export let as: keyof HTMLElementTagNameMap = 'div';
  export let className = '';

  const base = 'rounded-xl';
  const variants: Record<Variant, string> = {
    elevated: 'card',
    outline: 'card card-outline',
    ghost: 'bg-transparent'
  };
  const paddings: Record<Padding, string> = {
    sm: 'p-3',
    md: 'p-4',
    lg: 'p-6'
  };

  $: classes = `${base} ${variants[variant]} ${paddings[padding]} ${className}`.trim();
</script>

<svelte:element this={as} class={classes} {...$$restProps}>
  <slot />
</svelte:element>
