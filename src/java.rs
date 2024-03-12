mod auto {
    // Make current crate available as `duchess` for use by the generated code.
    // NB. in doctests, the current crate is already available as duchess.
    #[cfg(not(doctest))]
    use crate as duchess;

    duchess_macro::java_package! {
        package java.lang;

        public class java.lang.Object {
            public java.lang.Object();
            public native int hashCode();
            public boolean equals(java.lang.Object);
            public java.lang.String toString();
            public final native void notify();
            public final native void notifyAll();
        }

        public class java.lang.Throwable {
            public java.lang.Throwable();
            public java.lang.String getMessage();
            public java.lang.String getLocalizedMessage();
            public synchronized java.lang.Throwable getCause();
            public synchronized java.lang.Throwable initCause(java.lang.Throwable);
            public java.lang.String toString();
            public void printStackTrace();
            public synchronized java.lang.Throwable fillInStackTrace();
            public java.lang.StackTraceElement[] getStackTrace();
            public void setStackTrace(java.lang.StackTraceElement[]);
            public final synchronized void addSuppressed(java.lang.Throwable);
            public final synchronized java.lang.Throwable[] getSuppressed();
        }

        public final class java.lang.StackTraceElement {
            public java.lang.StackTraceElement(java.lang.String, java.lang.String, java.lang.String, int);
            public java.lang.String getFileName();
            public int getLineNumber();
            public java.lang.String getModuleName();
            public java.lang.String getModuleVersion();
            public java.lang.String getClassLoaderName();
            public java.lang.String getClassName();
            public java.lang.String getMethodName();
            public boolean isNativeMethod();
            public java.lang.String toString();
            public boolean equals(java.lang.Object);
            public int hashCode();
        }

        public class java.lang.Exception extends java.lang.Throwable {
            public java.lang.Exception();
        }

        public class java.lang.RuntimeException extends java.lang.Exception {
            public java.lang.RuntimeException();
        }

        // NB: In Java, this is `Class<T>`, but we model it as the erased version
        // `Class`. This is beacuse there are a lot of methods, including some that we would
        // like to model such as `arrayType()`, that return a `Class<?>`, and we cannot model
        // `?` in return position. By erasing the type parameter, we permit users to just
        // write `java.lang.Class` for those methods, but this does mean that some of the fancier
        // reflection types in Java won't work.
        //
        // FIXME(#41): It's not clear that this is the best solution, and we may revisit it in the future,
        // perhaps by not modeling `arrayType()` and friends, or perhaps by finding some way to
        // model `?` in return types in a satisfactory way.
        public final class java.lang.Class {
            public java.lang.String toString();
            public java.lang.String toGenericString();
            public native boolean isInstance(java.lang.Object);
            public native boolean isInterface();
            public native boolean isArray();
            public native boolean isPrimitive();
            public boolean isAnnotation();
            public boolean isSynthetic();
            public java.lang.String getName();
            public native java.lang.Class getSuperclass();
            // public native boolean isAssignableFrom(java.lang.Class<?>);
            public java.lang.String getPackageName();
            public java.lang.Class[] getInterfaces();
            public java.lang.Class getComponentType();
            public java.lang.Class arrayType();
        }

        public final class java.lang.String {
            public java.lang.String(byte[]);
            public int length();
            public boolean isEmpty();
        }

        public abstract class java.lang.Record {
            public abstract boolean equals(java.lang.Object);
            public abstract int hashCode();
            public abstract java.lang.String toString();
        }

        public interface java.lang.Runnable {
            public abstract void run();
        }

        package java.util;

        public interface java.util.List<E> {
            public abstract int size();
            public abstract boolean isEmpty();
            public abstract boolean contains(java.lang.Object);
            public abstract <T> T[] toArray(T[]);
            public abstract boolean add(E);
            public abstract boolean remove(java.lang.Object);
            public abstract void clear();
            public abstract boolean equals(java.lang.Object);
            public abstract int hashCode();
            public abstract E get(int);
            public abstract E set(int, E);
            public abstract int indexOf(java.lang.Object);
            public abstract int lastIndexOf(java.lang.Object);
            public abstract java.util.List<E> subList(int, int);

            // FIXME: Java generics from static methods are totally
            // disjoint from the enclosing Self type, but not in Rust.
            // How do we bridge this gap most ergonomically?
            //
            // public static <E> java.util.List<E> of(E...);
        }

        public class java.util.ArrayList<E> implements java.util.List<E> {
            public java.util.ArrayList();
            public void trimToSize();
            public void ensureCapacity(int);
            public int size();
            public boolean isEmpty();
            public boolean contains(java.lang.Object);
            public int indexOf(java.lang.Object);
            public int lastIndexOf(java.lang.Object);
            public java.lang.Object clone();
            public java.lang.Object[] toArray();
            public E get(int);
            public E set(int, E);
            public boolean add(E);
            public boolean equals(java.lang.Object);
            public int hashCode();
            public boolean remove(java.lang.Object);
            public void clear();
            public java.util.List<E> subList(int, int);
        }

        public interface java.util.Map<K, V> {
            public abstract int size();
            public abstract boolean isEmpty();
            public abstract boolean containsKey(java.lang.Object);
            public abstract boolean containsValue(java.lang.Object);
            public abstract V get(java.lang.Object);
            public abstract V put(K, V);
            public abstract V remove(java.lang.Object);
            public abstract void putAll(java.util.Map<? extends K, ? extends V>);
            public abstract void clear();
            // public abstract java.util.Set<K> keySet();
            // public abstract java.util.Collection<V> values();
            // public abstract java.util.Set<java.util.Map$Entry<K, V>> entrySet();
            public abstract boolean equals(java.lang.Object);
            public abstract int hashCode();
            public default V getOrDefault(java.lang.Object, V);
            // public default void forEach(java.util.function.BiConsumer<? super K, ? super V>);
            // public default void replaceAll(java.util.function.BiFunction<? super K, ? super V, ? extends V>);
            public default V putIfAbsent(K, V);
            // public default boolean remove(java.lang.Object, java.lang.Object);
            // public default boolean replace(K, V, V);
            // public default V replace(K, V);
            // // public default V computeIfAbsent(K, java.util.function.Function<? super K, ? extends V>);
            // public default V computeIfPresent(K, java.util.function.BiFunction<? super K, ? super V, ? extends V>);
            // public default V compute(K, java.util.function.BiFunction<? super K, ? super V, ? extends V>);
            // public default V merge(K, V, java.util.function.BiFunction<? super V, ? super V, ? extends V>);
            // public static <K, V> java.util.Map<K, V> of();
            // public static <K, V> java.util.Map<K, V> of(K, V);
            // public static <K, V> java.util.Map<K, V> of(K, V, K, V);
            // public static <K, V> java.util.Map<K, V> of(K, V, K, V, K, V);
            // public static <K, V> java.util.Map<K, V> of(K, V, K, V, K, V, K, V);
            // public static <K, V> java.util.Map<K, V> of(K, V, K, V, K, V, K, V, K, V);
            // public static <K, V> java.util.Map<K, V> of(K, V, K, V, K, V, K, V, K, V, K, V);
            // public static <K, V> java.util.Map<K, V> of(K, V, K, V, K, V, K, V, K, V, K, V, K, V);
            // public static <K, V> java.util.Map<K, V> of(K, V, K, V, K, V, K, V, K, V, K, V, K, V, K, V);
            // public static <K, V> java.util.Map<K, V> of(K, V, K, V, K, V, K, V, K, V, K, V, K, V, K, V, K, V);
            // public static <K, V> java.util.Map<K, V> of(K, V, K, V, K, V, K, V, K, V, K, V, K, V, K, V, K, V, K, V);
            // public static <K, V> java.util.Map<K, V> ofEntries(java.util.Map$Entry<? extends K, ? extends V>...);
            // public static <K, V> java.util.Map$Entry<K, V> entry(K, V);
            // public static <K, V> java.util.Map<K, V> copyOf(java.util.Map<? extends K, ? extends V>);
        }

        public class java.util.HashMap<K, V>
            // extends java.util.AbstractMap<K, V>
            implements java.util.Map<K, V> // , java.lang.Cloneable, java.io.Serializable
        {
            // public java.util.HashMap(int, float);
            // public java.util.HashMap(int);
            public java.util.HashMap();
            // public java.util.HashMap(java.util.Map<? extends K, ? extends V>);
            public int size();
            public boolean isEmpty();
            public V get(java.lang.Object);
            public boolean containsKey(java.lang.Object);
            public V put(K, V);
            public void putAll(java.util.Map<? extends K, ? extends V>);
            public V remove(java.lang.Object);
            public void clear();
            public boolean containsValue(java.lang.Object);
            // public java.util.Set<K> keySet();
            // public java.util.Collection<V> values();
            // public java.util.Set<java.util.Map$Entry<K, V>> entrySet();
            public V getOrDefault(java.lang.Object, V);
            public V putIfAbsent(K, V);
            // public boolean remove(java.lang.Object, java.lang.Object);
            // public boolean replace(K, V, V);
            // public V replace(K, V);
            // public V computeIfAbsent(K, java.util.function.Function<? super K, ? extends V>);
            // public V computeIfPresent(K, java.util.function.BiFunction<? super K, ? super V, ? extends V>);
            // public V compute(K, java.util.function.BiFunction<? super K, ? super V, ? extends V>);
            // public V merge(K, V, java.util.function.BiFunction<? super V, ? super V, ? extends V>);
            // public void forEach(java.util.function.BiConsumer<? super K, ? super V>);
            // public void replaceAll(java.util.function.BiFunction<? super K, ? super V, ? extends V>);
            public java.lang.Object clone();
        }

        public class java.util.Date { // implements java.io.Serializable, java.lang.Cloneable, java.lang.Comparable<java.util.Date> {
            public java.util.Date();
            //   public java.util.Date(long);
            //   public java.util.Date(int, int, int);
            //   public java.util.Date(int, int, int, int, int);
            //   public java.util.Date(int, int, int, int, int, int);
            //   public java.util.Date(java.lang.String);
            // public java.lang.Object clone();
            public static long UTC(int, int, int, int, int, int);
            public static long parse(java.lang.String);
            public int getYear();
            public void setYear(int);
            public int getMonth();
            public void setMonth(int);
            public int getDate();
            public void setDate(int);
            public int getDay();
            public int getHours();
            public void setHours(int);
            public int getMinutes();
            public void setMinutes(int);
            public int getSeconds();
            public void setSeconds(int);
            public long getTime();
            public void setTime(long);
            public boolean before(java.util.Date);
            public boolean after(java.util.Date);
            public boolean equals(java.lang.Object);
            // static final long getMillisOf(java.util.Date);
            public int compareTo(java.util.Date);
            public int hashCode();
            public java.lang.String toString();
            public java.lang.String toLocaleString();
            public java.lang.String toGMTString();
            public int getTimezoneOffset();
            // public static java.util.Date from(java.time.Instant);
            // public java.time.Instant toInstant();
            // public int compareTo(java.lang.Object);
            //   static {};
        }

        package java.time;

        public final class java.time.Instant {
            public static final java.time.Instant EPOCH;
            public static final java.time.Instant MIN;
            public static final java.time.Instant MAX;
            public static java.time.Instant now();
            // public static java.time.Instant now(java.time.Clock);
            // public static java.time.Instant ofEpochSecond(long);
            public static java.time.Instant ofEpochSecond(long, long);
            public static java.time.Instant ofEpochMilli(long);
            // public static java.time.Instant from(java.time.temporal.TemporalAccessor);
            // public static java.time.Instant parse(java.lang.CharSequence);
            // public boolean isSupported(java.time.temporal.TemporalField);
            // public boolean isSupported(java.time.temporal.TemporalUnit);
            // public java.time.temporal.ValueRange range(java.time.temporal.TemporalField);
            // public int get(java.time.temporal.TemporalField);
            // public long getLong(java.time.temporal.TemporalField);
            public long getEpochSecond();
            public int getNano();
            // public java.time.Instant with(java.time.temporal.TemporalAdjuster);
            // public java.time.Instant with(java.time.temporal.TemporalField, long);
            // public java.time.Instant truncatedTo(java.time.temporal.TemporalUnit);
            // public java.time.Instant plus(java.time.temporal.TemporalAmount);
            // public java.time.Instant plus(long, java.time.temporal.TemporalUnit);
            public java.time.Instant plusSeconds(long);
            public java.time.Instant plusMillis(long);
            public java.time.Instant plusNanos(long);
            // public java.time.Instant minus(java.time.temporal.TemporalAmount);
            // public java.time.Instant minus(long, java.time.temporal.TemporalUnit);
            public java.time.Instant minusSeconds(long);
            public java.time.Instant minusMillis(long);
            public java.time.Instant minusNanos(long);
            // public <R> R query(java.time.temporal.TemporalQuery<R>);
            // public java.time.temporal.Temporal adjustInto(java.time.temporal.Temporal);
            // public long until(java.time.temporal.Temporal, java.time.temporal.TemporalUnit);
            // public java.time.OffsetDateTime atOffset(java.time.ZoneOffset);
            // public java.time.ZonedDateTime atZone(java.time.ZoneId);
            public long toEpochMilli();
            public int compareTo(java.time.Instant);
            public boolean isAfter(java.time.Instant);
            public boolean isBefore(java.time.Instant);
            public boolean equals(java.lang.Object);
            public int hashCode();
            public java.lang.String toString();
            // void writeExternal(java.io.DataOutput) throws java.io.IOException;
            // static java.time.Instant readExternal(java.io.DataInput) throws java.io.IOException;
            // public java.time.temporal.Temporal minus(long, java.time.temporal.TemporalUnit);
            // public java.time.temporal.Temporal minus(java.time.temporal.TemporalAmount);
            // public java.time.temporal.Temporal plus(long, java.time.temporal.TemporalUnit);
            // public java.time.temporal.Temporal plus(java.time.temporal.TemporalAmount);
            // public java.time.temporal.Temporal with(java.time.temporal.TemporalField, long);
            // public java.time.temporal.Temporal with(java.time.temporal.TemporalAdjuster);
            // public int compareTo(java.lang.Object);
        }

        package java.io;

        class InputStream {
            // public java.io.InputStream();
            // public static java.io.InputStream nullInputStream();
            // public abstract int read() throws java.io.IOException;
            // public int read(byte[]) throws java.io.IOException;
            // public int read(byte[], int, int) throws java.io.IOException;
            // public byte[] readAllBytes() throws java.io.IOException;
            // public byte[] readNBytes(int) throws java.io.IOException;
            // public int readNBytes(byte[], int, int) throws java.io.IOException;
            // public long skip(long) throws java.io.IOException;
            // public void skipNBytes(long) throws java.io.IOException;
            // public int available() throws java.io.IOException;
            // public void close() throws java.io.IOException;
            // public void mark(int);
            // public void reset() throws java.io.IOException;
            // public boolean markSupported();
            // public long transferTo(java.io.OutputStream) throws java.io.IOException;
        }

        class FileInputStream extends java.io.InputStream {
            public java.io.FileInputStream(java.lang.String);
            // public java.io.FileInputStream(java.io.File) throws java.io.FileNotFoundException;
            // public java.io.FileInputStream(java.io.FileDescriptor);
            // public int read() throws java.io.IOException;
            // public int read(byte[]) throws java.io.IOException;
            // public int read(byte[], int, int) throws java.io.IOException;
            // public byte[] readAllBytes() throws java.io.IOException;
            // public byte[] readNBytes(int) throws java.io.IOException;
            // public long transferTo(java.io.OutputStream) throws java.io.IOException;
            // public long skip(long) throws java.io.IOException;
            // public int available() throws java.io.IOException;
            // public void close() throws java.io.IOException;
            // public final java.io.FileDescriptor getFD() throws java.io.IOException;
            // public java.nio.channels.FileChannel getChannel();
            // static {};
        }

        package java.nio;

        class java.nio.Buffer {
            // static final jdk.internal.misc.Unsafe UNSAFE;
            // static final jdk.internal.misc.ScopedMemoryAccess SCOPED_MEMORY_ACCESS;
            // static final int SPLITERATOR_CHARACTERISTICS;
            // long address;
            // final java.lang.foreign.MemorySegment segment;
            // static final boolean $assertionsDisabled;
            // java.nio.Buffer(long, int, java.lang.foreign.MemorySegment);
            // java.nio.Buffer(int, int, int, int, java.lang.foreign.MemorySegment);
            // static java.lang.IllegalArgumentException createSameBufferException();
            // static java.lang.IllegalArgumentException createCapacityException(int);
            public final int capacity();
            public final int position();
            // public java.nio.Buffer position(int);
            public final int limit();
            // public java.nio.Buffer limit(int);
            public java.nio.Buffer mark();
            // public java.nio.Buffer reset();
            // public java.nio.Buffer clear();
            public java.nio.Buffer flip();
            // public java.nio.Buffer rewind();
            // public final int remaining();
            // public final boolean hasRemaining();
            // public abstract boolean isReadOnly();
            // public abstract boolean hasArray();
            // public abstract java.lang.Object array();
            // public abstract int arrayOffset();
            // public abstract boolean isDirect();
            // public abstract java.nio.Buffer slice();
            // public abstract java.nio.Buffer slice(int, int);
            // public abstract java.nio.Buffer duplicate();
            // abstract java.lang.Object base();
            // final int nextGetIndex();
            // final int nextGetIndex(int);
            // final int nextPutIndex();
            // final int nextPutIndex(int);
            // final int checkIndex(int);
            // final int checkIndex(int, int);
            // final int markValue();
            // final void discardMark();
            // final jdk.internal.foreign.MemorySessionImpl session();
            // final void checkSession();
            // static {};
          }    

        public abstract class java.nio.ByteBuffer extends java.nio.Buffer { // implements java.lang.Comparable<java.nio.ByteBuffer>
            // final byte[] hb;
            // final int offset;
            // boolean isReadOnly;
            // boolean bigEndian;
            // boolean nativeByteOrder;
            // static final boolean $assertionsDisabled;
            // java.nio.ByteBuffer(int, int, int, int, byte[], int, java.lang.foreign.MemorySegment);
            // java.nio.ByteBuffer(int, int, int, int, java.lang.foreign.MemorySegment);
            // java.nio.ByteBuffer(byte[], long, int, java.lang.foreign.MemorySegment);
            // java.lang.Object base();
            // public static java.nio.ByteBuffer allocateDirect(int);
            public static java.nio.ByteBuffer allocate(int);
            // public static java.nio.ByteBuffer wrap(byte[], int, int);
            public static java.nio.ByteBuffer wrap(byte[]);
            public abstract java.nio.ByteBuffer slice();
            // public abstract java.nio.ByteBuffer slice(int, int);
            // public abstract java.nio.ByteBuffer duplicate();
            // public abstract java.nio.ByteBuffer asReadOnlyBuffer();
            // public abstract byte get();
            // public abstract java.nio.ByteBuffer put(byte);
            // public abstract byte get(int);
            // public abstract java.nio.ByteBuffer put(int, byte);
            // public java.nio.ByteBuffer get(byte[], int, int);
            // public java.nio.ByteBuffer get(byte[]);
            // public java.nio.ByteBuffer get(int, byte[], int, int);
            // public java.nio.ByteBuffer get(int, byte[]);
            // public java.nio.ByteBuffer put(java.nio.ByteBuffer);
            // public java.nio.ByteBuffer put(int, java.nio.ByteBuffer, int, int);
            // void putBuffer(int, java.nio.ByteBuffer, int, int);
            // public java.nio.ByteBuffer put(byte[], int, int);
            public final java.nio.ByteBuffer put(byte[]);
            // public java.nio.ByteBuffer put(int, byte[], int, int);
            // public java.nio.ByteBuffer put(int, byte[]);
            // java.nio.ByteBuffer putArray(int, byte[], int, int);
            // public final boolean hasArray();
            public final byte[] array();
            // public final int arrayOffset();
            public java.nio.ByteBuffer position(int);
            // public java.nio.ByteBuffer limit(int);
            // public java.nio.ByteBuffer mark();
            // public java.nio.ByteBuffer reset();
            public java.nio.ByteBuffer clear();
            // public java.nio.ByteBuffer flip();
            // public java.nio.ByteBuffer rewind();
            // public abstract java.nio.ByteBuffer compact();
            // public abstract boolean isDirect();
            // public java.lang.String toString();
            // public int hashCode();
            // public boolean equals(java.lang.Object);
            // public int compareTo(java.nio.ByteBuffer);
            // public int mismatch(java.nio.ByteBuffer);
            // public final java.nio.ByteOrder order();
            // public final java.nio.ByteBuffer order(java.nio.ByteOrder);
            // public final int alignmentOffset(int, int);
            // public final java.nio.ByteBuffer alignedSlice(int);
            // public abstract char getChar();
            // public abstract java.nio.ByteBuffer putChar(char);
            // public abstract char getChar(int);
            // public abstract java.nio.ByteBuffer putChar(int, char);
            // public abstract java.nio.CharBuffer asCharBuffer();
            // public abstract short getShort();
            // public abstract java.nio.ByteBuffer putShort(short);
            // public abstract short getShort(int);
            // public abstract java.nio.ByteBuffer putShort(int, short);
            // public abstract java.nio.ShortBuffer asShortBuffer();
            // public abstract int getInt();
            // public abstract java.nio.ByteBuffer putInt(int);
            // public abstract int getInt(int);
            // public abstract java.nio.ByteBuffer putInt(int, int);
            // public abstract java.nio.IntBuffer asIntBuffer();
            // public abstract long getLong();
            // public abstract java.nio.ByteBuffer putLong(long);
            // public abstract long getLong(int);
            // public abstract java.nio.ByteBuffer putLong(int, long);
            // public abstract java.nio.LongBuffer asLongBuffer();
            // public abstract float getFloat();
            // public abstract java.nio.ByteBuffer putFloat(float);
            // public abstract float getFloat(int);
            // public abstract java.nio.ByteBuffer putFloat(int, float);
            // public abstract java.nio.FloatBuffer asFloatBuffer();
            // public abstract double getDouble();
            // public abstract java.nio.ByteBuffer putDouble(double);
            // public abstract double getDouble(int);
            // public abstract java.nio.ByteBuffer putDouble(int, double);
            // public abstract java.nio.DoubleBuffer asDoubleBuffer();
            // public java.nio.Buffer duplicate();
            // public java.nio.Buffer slice(int, int);
            // public java.nio.Buffer slice();
            // public java.lang.Object array();
            // public java.nio.Buffer rewind();
            // public java.nio.Buffer flip();
            // public java.nio.Buffer clear();
            // public java.nio.Buffer reset();
            // public java.nio.Buffer mark();
            // public java.nio.Buffer limit(int);
            // public java.nio.Buffer position(int);
            // public int compareTo(java.lang.Object);
            // static {};
          }

        package java.security;
    
        class SecureRandom {
            //static final long serialVersionUID;
            public java.security.SecureRandom();
            // public java.security.SecureRandom(byte[]);
            // protected java.security.SecureRandom(java.security.SecureRandomSpi, java.security.Provider);
            // public static java.security.SecureRandom getInstance(java.lang.String) throws java.security.NoSuchAlgorithmException;
            // public static java.security.SecureRandom getInstance(java.lang.String, java.lang.String) throws java.security.NoSuchAlgorithmException, java.security.NoSuchProviderException;
            // public static java.security.SecureRandom getInstance(java.lang.String, java.security.Provider) throws java.security.NoSuchAlgorithmException;
            // public static java.security.SecureRandom getInstance(java.lang.String, java.security.SecureRandomParameters) throws java.security.NoSuchAlgorithmException;
            // public static java.security.SecureRandom getInstance(java.lang.String, java.security.SecureRandomParameters, java.lang.String) throws java.security.NoSuchAlgorithmException, java.security.NoSuchProviderException;
            // public static java.security.SecureRandom getInstance(java.lang.String, java.security.SecureRandomParameters, java.security.Provider) throws java.security.NoSuchAlgorithmException;
            // public final java.security.Provider getProvider();
            // public java.lang.String getAlgorithm();
            // public java.lang.String toString();
            // public java.security.SecureRandomParameters getParameters();
            // public void setSeed(byte[]);
            // public void setSeed(long);
            public void nextBytes(byte[]);
            // public void nextBytes(byte[], java.security.SecureRandomParameters);
            // protected final int next(int);
            // public static byte[] getSeed(int);
            // public byte[] generateSeed(int);
            // public static java.security.SecureRandom getInstanceStrong() throws java.security.NoSuchAlgorithmException;
            // public void reseed();
            // public void reseed(java.security.SecureRandomParameters);
            // static {};
        }

        class KeyStore {
            // protected java.security.KeyStore(java.security.KeyStoreSpi, java.security.Provider, java.lang.String);
            public static java.security.KeyStore getInstance(java.lang.String) throws java.security.KeyStoreException;
            // public static java.security.KeyStore getInstance(java.lang.String, java.lang.String) throws java.security.KeyStoreException, java.security.NoSuchProviderException;
            // public static java.security.KeyStore getInstance(java.lang.String, java.security.Provider) throws java.security.KeyStoreException;
            public static final java.lang.String getDefaultType();
            // public final java.security.Provider getProvider();
            // public final java.lang.String getType();
            // public final java.util.Set<java.security.KeyStore$Entry$Attribute> getAttributes(java.lang.String) throws java.security.KeyStoreException;
            // public final java.security.Key getKey(java.lang.String, char[]) throws java.security.KeyStoreException, java.security.NoSuchAlgorithmException, java.security.UnrecoverableKeyException;
            // public final java.security.cert.Certificate[] getCertificateChain(java.lang.String) throws java.security.KeyStoreException;
            // public final java.security.cert.Certificate getCertificate(java.lang.String) throws java.security.KeyStoreException;
            // public final java.util.Date getCreationDate(java.lang.String) throws java.security.KeyStoreException;
            // public final void setKeyEntry(java.lang.String, java.security.Key, char[], java.security.cert.Certificate[]) throws java.security.KeyStoreException;
            // public final void setKeyEntry(java.lang.String, byte[], java.security.cert.Certificate[]) throws java.security.KeyStoreException;
            public final void setCertificateEntry(java.lang.String, java.security.cert.Certificate) throws java.security.KeyStoreException;
            // public final void deleteEntry(java.lang.String) throws java.security.KeyStoreException;
            // public final java.util.Enumeration<java.lang.String> aliases() throws java.security.KeyStoreException;
            // public final boolean containsAlias(java.lang.String) throws java.security.KeyStoreException;
            // public final int size() throws java.security.KeyStoreException;
            // public final boolean isKeyEntry(java.lang.String) throws java.security.KeyStoreException;
            // public final boolean isCertificateEntry(java.lang.String) throws java.security.KeyStoreException;
            // public final java.lang.String getCertificateAlias(java.security.cert.Certificate) throws java.security.KeyStoreException;
            // public final void store(java.io.OutputStream, char[]) throws java.security.KeyStoreException, java.io.IOException, java.security.NoSuchAlgorithmException, java.security.cert.CertificateException;
            // public final void store(java.security.KeyStore$LoadStoreParameter) throws java.security.KeyStoreException, java.io.IOException, java.security.NoSuchAlgorithmException, java.security.cert.CertificateException;
            public final void load(java.io.InputStream, char[]) throws java.io.IOException, java.security.NoSuchAlgorithmException, java.security.cert.CertificateException;
            // public final void load(java.security.KeyStore$LoadStoreParameter) throws java.io.IOException, java.security.NoSuchAlgorithmException, java.security.cert.CertificateException;
            // public final java.security.KeyStore$Entry getEntry(java.lang.String, java.security.KeyStore$ProtectionParameter) throws java.security.NoSuchAlgorithmException, java.security.UnrecoverableEntryException, java.security.KeyStoreException;
            // public final void setEntry(java.lang.String, java.security.KeyStore$Entry, java.security.KeyStore$ProtectionParameter) throws java.security.KeyStoreException;
            // public final boolean entryInstanceOf(java.lang.String, java.lang.Class<? extends java.security.KeyStore$Entry>) throws java.security.KeyStoreException;
            // public static final java.security.KeyStore getInstance(java.io.File, char[]) throws java.security.KeyStoreException, java.io.IOException, java.security.NoSuchAlgorithmException, java.security.cert.CertificateException;
            // public static final java.security.KeyStore getInstance(java.io.File, java.security.KeyStore$LoadStoreParameter) throws java.security.KeyStoreException, java.io.IOException, java.security.NoSuchAlgorithmException, java.security.cert.CertificateException;
            // static {};
        }

        package java.security.cert;

        class CertificateFactory {
            public static final java.security.cert.CertificateFactory getInstance(java.lang.String) throws java.security.cert.CertificateException;
            public final java.security.cert.Certificate generateCertificate(java.io.InputStream) throws java.security.cert.CertificateException;
        }

        class Certificate {

        }

    }
}

// pub struct NullObject { }
    
// impl<'a> duchess::IntoJava<java::security::SecureRandom> for MyNull {
//     type Output<'jvm>: AsJRef<T>;

//     fn into_java<'jvm>(self, jvm: &mut duchess::Jvm<'jvm>) -> duchess::Result<'jvm, Self::Output<'jvm>> {
//         todo!()
//     }
// }


pub use auto::java::*;

// XX this isn't a real class in the JVM, since each array type (e.g. Foo[] and int[]) is just a subclass of Object.
// Should it go somewhere outside of the JDK core classes?
pub use crate::array::JavaArray as Array;
pub use crate::array::JavaArrayExt as ArrayExt;
