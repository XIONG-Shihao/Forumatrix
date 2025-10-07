import React from 'react';
import type { PostListItem } from '../../features/posts/api';
import { formatUnix } from '../../features/posts/api';

export function PostMiniCard({
  post,
  onClick,
}: {
  post: PostListItem;
  onClick?: (id: number) => void;
}) {
  const when = formatUnix(post.created_at);
  const preview =
    post.body.length > 200 ? post.body.slice(0, 200) + '…' : post.body;

  return (
    <article
      className="card"
      onClick={() => onClick?.(post.id)}
      style={{
        padding: 12,
        cursor: onClick ? 'pointer' : 'default',
        position: 'relative',
      }}
    >
      <div
        style={{
          position: 'absolute',
          right: 12,
          top: 10,
          fontSize: 12,
          color: '#9aa3c6',
        }}
      >
        {when}
      </div>
      <h3 style={{ margin: '4px 0 6px', lineHeight: 1.25 }}>{post.title}</h3>
      <p style={{ margin: 0, opacity: 0.9, lineHeight: 1.45 }}>{preview}</p>
    </article>
  );
}
