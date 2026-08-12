import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { cn } from '@/lib/utils';
import { Icons } from './ui/icons';

const navigation = [
  { name: 'Dashboard', href: '/', icon: Icons.shield },
  { name: 'Reputation', href: '/reputation', icon: Icons.eye },
  { name: 'Reports', href: '/reports', icon: Icons.file },
  { name: 'Research', href: '/research', icon: Icons.users },
  { name: 'Settings', href: '/settings', icon: Icons.settings },
];

export function Sidebar() {
  const location = useLocation();

  return (
    <div className="flex h-full w-64 flex-col border-r bg-card">
      <div className="flex h-16 items-center border-b px-6">
        <Link to="/" className="flex items-center gap-2 font-bold text-xl">
          <Icons.shield className="h-8 w-8 text-primary" />
          <span>M'Eye</span>
        </Link>
      </div>
      
      <nav className="flex-1 space-y-1 p-4">
        {navigation.map((item) => {
          const Icon = item.icon;
          const isActive = location.pathname === item.href;
          
          return (
            <Link
              key={item.name}
              to={item.href}
              className={cn(
                'flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors',
                isActive
                  ? 'bg-primary text-primary-foreground'
                  : 'text-muted-foreground hover:bg-accent hover:text-accent-foreground'
              )}
            >
              <Icon className="h-5 w-5" />
              {item.name}
            </Link>
          );
        })}
      </nav>
      
      <div className="border-t p-4">
        <button className="flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm font-medium text-muted-foreground hover:bg-accent hover:text-accent-foreground">
          <Icons.logout className="h-5 w-5" />
          Logout
        </button>
      </div>
    </div>
  );
}
