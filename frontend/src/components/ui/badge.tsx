import * as React from 'react';
import { cn } from '@/lib/utils';

export interface BadgeProps extends React.HTMLAttributes<HTMLDivElement> {
  variant?: 'default' | 'secondary' | 'destructive' | 'outline' | 'safe' | 'watch' | 'blocked' | 'compromised' | 'unknown';
}

const Badge = React.forwardRef<HTMLDivElement, BadgeProps>(
  ({ className, variant = 'default', ...props }, ref) => {
    const baseStyles = 'inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2';
    
    const variants = {
      default: 'border-transparent bg-primary text-primary-foreground shadow hover:bg-primary/80',
      secondary: 'border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80',
      destructive: 'border-transparent bg-destructive text-destructive-foreground shadow hover:bg-destructive/80',
      outline: 'text-foreground',
      safe: 'border-transparent bg-green-500 text-white',
      watch: 'border-transparent bg-orange-500 text-white',
      blocked: 'border-transparent bg-red-500 text-white',
      compromised: 'border-transparent bg-purple-500 text-white',
      unknown: 'border-transparent bg-gray-400 text-white',
    };
    
    return (
      <div ref={ref} className={cn(baseStyles, variants[variant], className)} {...props} />
    );
  }
);
Badge.displayName = 'Badge';

export { Badge };
