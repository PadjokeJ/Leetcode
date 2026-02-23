class Slow {
    public boolean hasAllCodes(String s, int k) {
        if (s.length() < 2 * k)
            return false;

        for (int i = 0; i < Math.pow(2, k); i++) {
            String b = Integer.toBinaryString(i);
            b = new StringBuilder(k).repeat("0", k - b.length()).append(b).toString();

            if (!s.contains(b))
                return false;
        }
        return true;
    }
}
