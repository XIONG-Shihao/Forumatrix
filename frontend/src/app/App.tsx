// frontend/src/App.tsx
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import AuthPage from '../features/auth/auth_page';
import Home from '../pages/Home';
import ProfilePage from '../pages/ProfilePage';
import PostPage from '../pages/PostPage';
import PublicProfilePage from '../pages/PublicProfilePage';
import RequireAuth from '../components/auth/RequireAuth';

// Docs
import DocsHome from '../pages/docs/DocsHome';
// If you haven't built this yet, create a stub to avoid build errors:
import DocPage from '../pages/docs/DocPage'; // TODO: implement next

export function App() {
  return (
    <BrowserRouter>
      <Routes>
        {/* Public */}
        <Route path="/auth" element={<AuthPage />} />

        {/* Everything below requires login */}
        <Route element={<RequireAuth />}>
          <Route path="/" element={<Home />} />
          <Route path="/home" element={<Home />} />
          <Route path="/profile" element={<ProfilePage />} />
          <Route path="/posts/:id" element={<PostPage />} />
          <Route path="/users/:id" element={<PublicProfilePage />} />

          {/* ---- Docs (protected) ---- */}
          <Route path="/docs" element={<DocsHome />} />
          <Route path="/docs/:docId" element={<DocPage />} />

          {/* catch-all inside guard */}
          <Route path="*" element={<Navigate to="/home" replace />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}