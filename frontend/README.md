# M'Eye Frontend

React + TypeScript + Vite frontend for M'Eye Email Security Platform.

## 🚀 Quick Start

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## 📁 Project Structure

```
frontend/
├── src/
│   ├── components/
│   │   ├── ui/           # Reusable UI components (Button, Input, Card, etc.)
│   │   ├── layout/       # Layout components (Sidebar, Header, DashboardLayout)
│   │   └── dashboard/    # Dashboard-specific components
│   ├── pages/            # Page components (Login, Dashboard, etc.)
│   ├── services/         # API services
│   ├── store/            # Zustand state management
│   ├── types/            # TypeScript type definitions
│   ├── lib/              # Utility functions
│   ├── hooks/            # Custom React hooks
│   └── App.tsx           # Main application component
├── public/               # Static assets
├── index.html
├── vite.config.ts
├── tailwind.config.js
└── package.json
```

## 🛠️ Tech Stack

- **React 18** - UI library
- **TypeScript** - Type safety
- **Vite** - Build tool
- **Tailwind CSS** - Styling
- **Radix UI** - Accessible components
- **Zustand** - State management
- **React Router** - Routing
- **Axios** - HTTP client
- **Recharts** - Charts & graphs
- **Lucide** - Icons

## 🔌 API Integration

The frontend connects to the backend API at `/api/v1`. During development, Vite proxies requests to `http://localhost:8000`.

### Environment Variables

Create a `.env` file in the frontend directory:

```env
VITE_API_URL=http://localhost:8000/api/v1
```

## 🎨 UI Components

### Button

```tsx
import { Button } from '@/components/ui/button';

<Button variant="default">Click me</Button>
<Button variant="destructive">Delete</Button>
<Button variant="outline">Cancel</Button>
```

### Input

```tsx
import { Input } from '@/components/ui/input';

<Input type="email" placeholder="Email" />
```

### Card

```tsx
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';

<Card>
  <CardHeader>
    <CardTitle>Title</CardTitle>
  </CardHeader>
  <CardContent>Content</CardContent>
</Card>
```

### Badge

```tsx
import { Badge } from '@/components/ui/badge';

<Badge variant="safe">SAFE</Badge>
<Badge variant="watch">WATCH</Badge>
<Badge variant="blocked">BLOCKED</Badge>
```

## 📊 State Management

State is managed using Zustand stores:

```tsx
import { useAuthStore } from '@/store';

function MyComponent() {
  const user = useAuthStore((state) => state.user);
  const setUser = useAuthStore((state) => state.setUser);
}
```

Available stores:
- `useAuthStore` - Authentication state
- `useDashboardStore` - Dashboard statistics
- `useReputationStore` - Reputation data
- `useReportStore` - Reports
- `useModerationStore` - Moderation queue

## 🔐 Authentication

The app uses JWT authentication with automatic token refresh and route protection.

```tsx
import { authService } from '@/services/api';

// Login
const response = await authService.login({ email, password });
localStorage.setItem('access_token', response.access_token);

// Logout
authService.logout();
```

Protected routes automatically redirect to `/login` if no token is present.

## 🚢 Docker

Build and run with Docker:

```bash
docker build -t m-eye-frontend .
docker run -p 3000:80 m-eye-frontend
```

Or use docker-compose from the project root:

```bash
docker compose up frontend
```

## 📝 Code Style

- ESLint configured for React + TypeScript
- Prettier for code formatting
- Tailwind CSS class ordering

Run linting:

```bash
npm run lint
```

## 🧪 Testing

Testing setup coming in Phase 2 with Vitest + React Testing Library.

## 📄 License

AGPL-3.0 - See LICENSE file for details.
