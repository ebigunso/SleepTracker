<script lang="ts">
  type Variant = 'neutral' | 'primary' | 'success' | 'warning' | 'info';
  type Size = 'sm' | 'md';

  export let variant: Variant = 'neutral';
  export let size: Size = 'md';
  export let className = '';

  const base = 'chip';
  const variants: Record<Variant, string> = {
    neutral: 'chip-neutral',
    primary: 'chip-primary',
    success: 'chip-success',
    warning: 'chip-warning',
    info: 'chip-info'
  };
  const sizes: Record<Size, string> = {
    sm: 'px-2 py-0.5 text-[11px]',
    md: 'px-2.5 py-1 text-xs'
  };

  $: classes = `${base} ${variants[variant]} ${sizes[size]} ${className}`.trim();
</script>

<span class={classes} {...$$restProps}>
  <slot />
</span>
