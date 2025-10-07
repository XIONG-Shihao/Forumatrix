import React, { useEffect, useState } from 'react';

type Props = {
  onUpload: (file: File) => Promise<void>;
  accept?: string;
  maxInfo?: string; // little helper text
};

export const AvatarUploader: React.FC<Props> = ({
  onUpload,
  accept = 'image/png, image/jpeg',
  maxInfo = 'PNG or JPEG, up to 2 MB.',
}) => {
  const [file, setFile] = useState<File | null>(null);
  const [previewUrl, setPreviewUrl] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [err, setErr] = useState<string | null>(null);

  useEffect(() => {
    if (!file) {
      setPreviewUrl(null);
      return;
    }
    const url = URL.createObjectURL(file);
    setPreviewUrl(url);
    return () => URL.revokeObjectURL(url);
  }, [file]);

  const handleUpload = async () => {
    if (!file) return;
    setErr(null);
    setLoading(true);
    try {
      await onUpload(file);
      setFile(null);
      setPreviewUrl(null);
    } catch (e: any) {
      setErr(e?.message ?? 'Upload failed');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="uploader">
      <div className="uploader-row">
        <input
          type="file"
          accept={accept}
          onChange={(e) => setFile(e.target.files?.[0] ?? null)}
        />
        <button
          className="btn"
          onClick={handleUpload}
          disabled={!file || loading}
        >
          {loading ? 'Uploading…' : 'Upload'}
        </button>
        <span className="hint">{maxInfo}</span>
      </div>

      {previewUrl && (
        <div className="uploader-preview">
          <div className="subtle">Preview</div>
          <img
            src={previewUrl}
            alt="Selected preview"
            className="uploader-preview-img"
          />
        </div>
      )}

      {err && <div className="error">{err}</div>}
    </div>
  );
};
