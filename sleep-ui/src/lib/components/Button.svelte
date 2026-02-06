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
    'inline-flex items-center justify-center font-semibold rounded-md transition-colors focus-ring disabled:opacity-60 disabled:cursor-not-allowed';
  const variants: Record<Variant, string> = {
    primary: 'btn-primary shadow-sm',
    secondary: 'btn-secondary',
    outline: 'btn-outline',
    ghost: 'btn-ghost',
    danger: 'btn-danger shadow-sm'
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
