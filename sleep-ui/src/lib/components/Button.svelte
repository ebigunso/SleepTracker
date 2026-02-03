<script lang="ts">
  type Variant = 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger';
  type Size = 'sm' | 'md' | 'lg';

  export let variant: Variant = 'primary';
  export let size: Size = 'md';
  export let fullWidth = false;
  export let type: 'button' | 'submit' | 'reset' = 'button';
  export let disabled = false;
  export let className = '';

  const base =
    'inline-flex items-center justify-center font-semibold rounded-md transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500/40 disabled:opacity-60 disabled:cursor-not-allowed';
  const variants: Record<Variant, string> = {
    primary: 'bg-indigo-600 text-white shadow-sm hover:bg-indigo-500',
    secondary: 'bg-indigo-50 text-indigo-700 ring-1 ring-inset ring-indigo-200 hover:bg-indigo-100',
    outline: 'bg-white text-slate-700 ring-1 ring-inset ring-slate-200 hover:bg-slate-50',
    ghost: 'bg-transparent text-slate-600 hover:bg-slate-100',
    danger: 'bg-rose-500 text-white shadow-sm hover:bg-rose-600'
  };
  const sizes: Record<Size, string> = {
    sm: 'px-2.5 py-1.5 text-xs',
    md: 'px-3.5 py-2 text-sm',
    lg: 'px-4 py-2.5 text-base'
  };

  $: classes = `${base} ${variants[variant]} ${sizes[size]} ${fullWidth ? 'w-full' : ''} ${className}`.trim();
</script>

<button {type} {disabled} class={classes} {...$$restProps}>
  <slot />
</button>
