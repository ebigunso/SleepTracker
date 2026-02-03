<script lang="ts">
  type Variant = 'neutral' | 'primary' | 'success' | 'warning' | 'info';
  type Size = 'sm' | 'md';

  export let variant: Variant = 'neutral';
  export let size: Size = 'md';
  export let className = '';

  const base = 'inline-flex items-center rounded-full font-semibold';
  const variants: Record<Variant, string> = {
    neutral: 'bg-slate-100 text-slate-700 ring-1 ring-inset ring-slate-200',
    primary: 'bg-indigo-100 text-indigo-700 ring-1 ring-inset ring-indigo-200',
    success: 'bg-emerald-100 text-emerald-700 ring-1 ring-inset ring-emerald-200',
    warning: 'bg-amber-100 text-amber-700 ring-1 ring-inset ring-amber-200',
    info: 'bg-sky-100 text-sky-700 ring-1 ring-inset ring-sky-200'
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
