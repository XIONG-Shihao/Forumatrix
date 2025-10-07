export type UserPublic = {
  id: number;
  email: string;
  username: string;
  dob?: string | null;
  bio?: string | null;
  is_active: number;
  is_admin: number;
  avatar_url?: string | null;
};

// new: update payload & call
export type UpdateUserBody = {
  username: string;
  dob?: string | null; // ISO "YYYY-MM-DD" or null
  bio?: string | null;
};
