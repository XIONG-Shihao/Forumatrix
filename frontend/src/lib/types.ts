export type UserPublic = {
  id: number;
  email: string;
  username: string;
  dob?: string | null;
  bio?: string | null;
  is_active: number;
  is_admin: number;
  avatar_url?: string; // 👈 backend should include this (or we'll fallback)
};
