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
    'block w-full rounded-md border bg-white text-sm text-slate-900 shadow-sm transition focus:outline-none focus:ring-2 focus:ring-indigo-500/40 focus:border-indigo-500 disabled:bg-slate-50 disabled:text-slate-500';
  const variants: Record<Variant, string> = {
    default: 'border-slate-200',
    ghost: 'border-transparent bg-slate-50',
    error: 'border-rose-300 text-rose-700 focus:border-rose-500 focus:ring-rose-400/40'
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
