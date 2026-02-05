<script lang="ts">
  type ElementType = 'input' | 'textarea' | 'select';
  type Variant = 'default' | 'ghost' | 'error';
  type Size = 'sm' | 'md' | 'lg';

  export let as: ElementType = 'input';
  export let variant: Variant = 'default';
  export let size: Size = 'md';
  export let value: string | number | undefined = undefined;
  export let type = 'text';
  export let rows = 3;
  export let className = '';

  const base =
    'input-base text-sm';
  const variants: Record<Variant, string> = {
    default: '',
    ghost: 'input-ghost',
    error: 'input-error'
  };
  const sizes: Record<Size, string> = {
    sm: 'px-2.5 py-1.5 text-xs',
    md: 'px-3 py-2 text-sm',
    lg: 'px-4 py-2.5 text-base'
  };

  $: classes = `${base} ${variants[variant]} ${sizes[size]} ${className}`.trim();
</script>

{#if as === 'textarea'}
  <textarea class={classes} rows={rows} bind:value {...$$restProps}>
    <slot />
  </textarea>
{:else if as === 'select'}
  <select class={classes} bind:value {...$$restProps}>
    <slot />
  </select>
{:else}
  <input class={classes} type={type} bind:value {...$$restProps} />
{/if}
